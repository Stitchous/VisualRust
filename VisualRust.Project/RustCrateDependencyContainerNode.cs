using System;
using Microsoft.VisualStudio;
using Microsoft.VisualStudioTools.Project;

namespace VisualRust.Project
{
    internal class RustCrateDependencyContainerNode : HierarchyNode
    {
        internal const string DependenciesNodeVirtualName = "Dependencies";

        internal RustCrateDependencyContainerNode(ProjectNode root) : base(root)
        {
            ExcludeNodeFromScc = true;
        }

        public override int SortPriority => DefaultSortOrderNode.ReferenceContainerNode;

        public override string Caption => DependenciesNodeVirtualName;

        public override Guid ItemTypeGuid => VSConstants.GUID_ItemType_VirtualFolder;

        public override string Url => DependenciesNodeVirtualName;

        public override object GetIconHandle(bool open)
        {
            return ProjectMgr.ImageHandler.GetIconHandle(open
                ? (int) ProjectNode.ImageName.OpenReferenceFolder
                : (int) ProjectNode.ImageName.ReferenceFolder);
        }

        public override string GetEditLabel()
        {
            return null;
        }
    }
}