using Microsoft.VisualStudio;
using Microsoft.VisualStudio.Text;
using Microsoft.VisualStudio.Text.Editor;
using Microsoft.VisualStudio.TextManager.Interop;
using System;
using System.Collections.Generic;

namespace VisualRust
{
    class RustPrettyPrintCommandHandler : VSCommandTarget<VSConstants.VSStd2KCmdID>
    {
        public RustPrettyPrintCommandHandler(IVsTextView vsTextView, IWpfTextView textView) : base(vsTextView, textView)
        {
            buffer = textView.TextBuffer;
        }

        protected override IEnumerable<VSConstants.VSStd2KCmdID> SupportedCommands
        {
            get
            {
                yield return VSConstants.VSStd2KCmdID.FORMATDOCUMENT;
            }
        }

        protected override uint ConvertFromCommand(VSConstants.VSStd2KCmdID command)
        {
            return (uint)command;
        }

        protected override VSConstants.VSStd2KCmdID ConvertFromCommandId(uint id)
        {
            return (VSConstants.VSStd2KCmdID) id;
        }

        protected override bool Execute(VSConstants.VSStd2KCmdID command, uint options, IntPtr pvaIn, IntPtr pvaOut)
        {
            var snapshot = buffer.CurrentSnapshot;
            int start = TextView.Selection.Start.Position.Position;
            int end = TextView.Selection.End.Position.Position;

            switch (command)
            {
                case VSConstants.VSStd2KCmdID.FORMATDOCUMENT:

                    break;
            }
            return true;
        }

        private readonly ITextBuffer buffer;
    }
}
