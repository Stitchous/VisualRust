using Microsoft.VisualStudio;
using Microsoft.VisualStudio.Text;
using Microsoft.VisualStudio.Text.Editor;
using Microsoft.VisualStudio.TextManager.Interop;
using System;
using System.Collections.Generic;
using VisualRust.NitraIntegration;

namespace VisualRust
{
    class RustPrettyPrintCommandHandler : VSCommandTarget<VSConstants.VSStd2KCmdID>
    {
        public RustPrettyPrintCommandHandler(IVsTextView vsTextView, IWpfTextView textView) : base(vsTextView, textView)
        {
            _prettyPrinter = new RustPrettyPrinter(textView.TextBuffer);
        }

        protected override IEnumerable<VSConstants.VSStd2KCmdID> SupportedCommands
        {
            get
            {
                yield return VSConstants.VSStd2KCmdID.FORMATDOCUMENT;
                //yield return VSConstants.VSStd2KCmdID.FORMATSELECTION;
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
            switch (command)
            {
                case VSConstants.VSStd2KCmdID.FORMATDOCUMENT:
                    _prettyPrinter.PrettyPrint();
                    break;

                case VSConstants.VSStd2KCmdID.FORMATSELECTION:
                    _prettyPrinter.PrettyPrint(TextView.Selection);
                    break;
            }
            return true;
        }

        private readonly RustPrettyPrinter _prettyPrinter;
    }
}
