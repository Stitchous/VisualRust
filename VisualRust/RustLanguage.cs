using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.Package;
using Microsoft.VisualStudio.TextManager.Interop;
using Microsoft.VisualStudio;

namespace VisualRust
{
    internal sealed class RustLanguage : LanguageService
    {
        private LanguagePreferences preferences;

        internal class StubScanner : IScanner
        {
            private IVsTextBuffer m_buffer;
            string m_source;

            public StubScanner(IVsTextBuffer buffer)
            {
                m_buffer = buffer;
            }

            bool IScanner.ScanTokenAndProvideInfoAboutIt(TokenInfo tokenInfo, ref int state)
            {
                tokenInfo.Type = TokenType.Unknown;
                tokenInfo.Color = TokenColor.Text;
                return false;
            }

            void IScanner.SetSource(string source, int offset)
            {
                m_source = source.Substring(offset);
            }
        }

        internal class StubAuthoringScope : AuthoringScope
        {
            public override string GetDataTipText(int line, int col, out TextSpan span)
            {
                span = new TextSpan();
                return null;
            }

            public override Declarations GetDeclarations(IVsTextView view,
                                                         int line,
                                                         int col,
                                                         TokenInfo info,
                                                         ParseReason reason)
            {
                return null;
            }

            public override string Goto(VSConstants.VSStd97CmdID cmd, IVsTextView textView, int line, int col, out TextSpan span)
            {
                span = new TextSpan();
                return null;
            }

            public override Methods GetMethods(int line, int col, string name)
            {
                return null;
            }
        }


        public override LanguagePreferences GetLanguagePreferences()
        {
            if (preferences == null)
            {
                preferences = new LanguagePreferences(this.Site, typeof(RustLanguage).GUID, this.Name);
                preferences.EnableCodeSense = true;
                preferences.EnableMatchBraces = true;
                preferences.EnableCommenting = true;
                preferences.EnableShowMatchingBrace = true;
                preferences.EnableMatchBracesAtCaret = true;
                preferences.HighlightMatchingBraceFlags = _HighlightMatchingBraceFlags.HMB_USERECTANGLEBRACES;
                preferences.LineNumbers = true;
                preferences.MaxErrorMessages = 100;
                preferences.AutoOutlining = false;
                preferences.MaxRegionTime = 2000;
                preferences.ShowNavigationBar = true;
                preferences.EnableFormatSelection = true;

                preferences.AutoListMembers = false;
                preferences.EnableQuickInfo = false;
                preferences.ParameterInformation = false;
            }
            return preferences;
        }

        public override string GetFormatFilterList()
        {
            return "Rust File (*.rs)|*.rs";
        }

        public override IScanner GetScanner(IVsTextLines buffer)
        {
            return new StubScanner(buffer);
        }

        public override string Name
        {
            get { return "Rust"; }
        }

        public override AuthoringScope ParseSource(ParseRequest req)
        {
            return new StubAuthoringScope();
        }

        public override Source CreateSource(IVsTextLines buffer)
        {
            return new RustSource(this, buffer, base.GetColorizer(buffer));
        }
    }
}
