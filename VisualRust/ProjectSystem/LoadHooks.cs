﻿// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

using System;
using System.Collections.Generic;
using System.ComponentModel.Composition;
using System.Globalization;
using System.IO;
using System.Threading.Tasks;
using Microsoft.Common.Core;
using Microsoft.Common.Core.Enums;
using Microsoft.Common.Core.IO;
using Microsoft.Common.Core.Shell;
using Microsoft.VisualStudio.ProjectSystem;
using VisualRust.ProjectSystem.FileSystemMirroring.IO;
using VisualRust.ProjectSystem.FileSystemMirroring.Project;
using Microsoft.VisualStudio.Shell.Interop;
using Microsoft.Common.Core.Logging;
//#if VS14
using Microsoft.VisualStudio.ProjectSystem.Utilities;
using IThreadHandling = Microsoft.VisualStudio.ProjectSystem.IThreadHandling;
using Microsoft.VisualStudio;
//#endif
#if VS15
using Microsoft.VisualStudio.ProjectSystem.VS;
using IThreadHandling = Microsoft.VisualStudio.ProjectSystem.IProjectThreadingService;
#endif


namespace VisualRust.ProjectSystem
{
    internal sealed class LoadHooks
    {
        [Export(typeof(IFileSystemMirroringProjectTemporaryItems))]
        [AppliesTo(VisualRustPackage.UniqueCapability)]
        private FileSystemMirroringProject Project { get; }

        private readonly MsBuildFileSystemWatcher _fileWatcher;
        private readonly string _projectDirectory;
        private readonly IFileSystem _fileSystem = new FileSystem();
        private readonly IThreadHandling _threadHandling;
        private readonly UnconfiguredProject _unconfiguredProject;
        private readonly IEnumerable<Lazy<IVsProject>> _cpsIVsProjects;

        /// <summary>
        /// Backing field for the similarly named property.
        /// </summary>
        [ImportingConstructor]
        public LoadHooks(UnconfiguredProject unconfiguredProject
            , [ImportMany("Microsoft.VisualStudio.ProjectSystem.Microsoft.VisualStudio.Shell.Interop.IVsProject")] IEnumerable<Lazy<IVsProject>> cpsIVsProjects
            , IProjectLockService projectLockService
            , IThreadHandling threadHandling)
        {

            _unconfiguredProject = unconfiguredProject;
            _cpsIVsProjects = cpsIVsProjects;
            _threadHandling = threadHandling;

            _projectDirectory = unconfiguredProject.GetProjectDirectory();

            var log = new DummyLogger();

            unconfiguredProject.ProjectUnloading += ProjectUnloading;
            _fileWatcher = new MsBuildFileSystemWatcher(_projectDirectory, "*", 25, 1000, _fileSystem, new MsBuildFileSystemFilter(), log);
            _fileWatcher.Error += FileWatcherError;
            Project = new FileSystemMirroringProject(unconfiguredProject, projectLockService, _fileWatcher, null, log);
        }

        public static IVsPackage EnsurePackageLoaded(Guid guidPackage)
        {
            var shell = (IVsShell)VisualRustPackage.GetGlobalService(typeof(IVsShell));
            var guid = guidPackage;
            IVsPackage package;
            int hr = ErrorHandler.ThrowOnFailure(shell.IsPackageLoaded(ref guid, out package), VSConstants.E_FAIL);
            guid = guidPackage;
            if (hr != VSConstants.S_OK)
            {
                ErrorHandler.ThrowOnFailure(shell.LoadPackage(ref guid, out package), VSConstants.E_FAIL);
            }

            return package;
        }

        [AppliesTo(VisualRustPackage.UniqueCapability)]
//#if VS14
        [UnconfiguredProjectAutoLoad2(completeBy: UnconfiguredProjectLoadCheckpoint.CapabilitiesEstablished)]
//#else
//		[ProjectAutoLoad(startAfter: ProjectLoadCheckpoint.UnconfiguredProjectLocalCapabilitiesEstablished,
//						 completeBy: ProjectLoadCheckpoint.BeforeLoadInitialConfiguration,
//						 RequiresUIThread = false)]
//#endif
        public async Task InitializeProjectFromDiskAsync()
        {
            await Project.CreateInMemoryImport();
            _fileWatcher.Start();

            await _threadHandling.SwitchToUIThread();
            // Make sure package is loaded
            EnsurePackageLoaded(GuidList.VisualRustPkgGuid);

            // TODO: start watching the Cargo manifest
        }

        private void FileWatcherError(object sender, EventArgs args)
        {
            _fileWatcher.Error -= FileWatcherError;
            //VsAppShell.Current.DispatchOnUIThread(() => {
            //	foreach (var iVsProjectLazy in _cpsIVsProjects)
            //	{
            //		IVsProject iVsProject;
            //		try
            //		{
            //			iVsProject = iVsProjectLazy.Value;
            //		}
            //		catch (Exception)
            //		{
            //			continue;
            //		}

            //		if (iVsProject.AsUnconfiguredProject() != _unconfiguredProject)
            //		{
            //			continue;
            //		}

            //		var solution = VsAppShell.Current.GetGlobalService<IVsSolution>(typeof(SVsSolution));
            //		solution.CloseSolutionElement((uint)__VSSLNCLOSEOPTIONS.SLNCLOSEOPT_UnloadProject, (IVsHierarchy)iVsProject, 0);
            //		return;
            //	}
            //});
        }

        private async Task ProjectUnloading(object sender, EventArgs args)
        {
            //VsAppShell.Current.AssertIsOnMainThread();

            _unconfiguredProject.ProjectUnloading -= ProjectUnloading;
            _fileWatcher.Dispose();

            if (!_fileSystem.DirectoryExists(_projectDirectory))
            {
                return;
            }
        }
    }
}