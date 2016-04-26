using Microsoft.VisualStudio.Package;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TextManager.Interop;

namespace VisualRust
{
    internal class RustSource : Source
    {
        public RustSource(LanguageService service, IVsTextLines textLines, Colorizer colorizer) : base(service, textLines, colorizer)
        {
        }

        public override void ReformatSpan(EditArray mgr, TextSpan span)
        {
            base.ReformatSpan(mgr, span);
        }
    }
}
