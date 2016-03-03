using VisualRust.Grammar;
using System.ComponentModel.Composition;
using Microsoft.VisualStudio.Text;
using Microsoft.VisualStudio.Text.Classification;
using Microsoft.VisualStudio.Utilities;
using Microsoft.VisualStudio.Language.StandardClassification;

namespace VisualRust.Text
{
	[Export(typeof(IClassifierProvider))]
    [ContentType("rust")]
    
    public sealed class RustClassifierProvider : IClassifierProvider
	{
		[Import]
		private IStandardClassificationService standardClassificationService;

		public IClassifier GetClassifier(ITextBuffer buffer)
		{
			return buffer.Properties.GetOrCreateSingletonProperty(creator: () => new RustClassifier(standardClassificationService));
		}
	}
}
