using System;
using Microsoft.VisualStudioTools.Project;

namespace VisualRust.Project
{
    internal class RustCrateDependencyNode : HierarchyNode
    {
        private readonly Guid _itemTypeGuid;
        private readonly string _url;
        private readonly string _caption;

        internal RustCrateDependencyNode(ProjectNode root, ProjectElement element)
        {
        }

        public override string Caption => _caption;

        public override Guid ItemTypeGuid => _itemTypeGuid;

        public override string Url => _url;
    }
}