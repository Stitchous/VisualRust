namespace VisualRust.Text
{
	using System;
	using System.Diagnostics;
	using System.Collections.Generic;
	using System.ComponentModel.Composition;
	using Microsoft.VisualStudio.Text;
	using Microsoft.VisualStudio.Text.Classification;
	using Microsoft.VisualStudio.Text.Editor;
	using Microsoft.VisualStudio.Text.Tagging;
	using Microsoft.VisualStudio.Utilities;
	using Microsoft.VisualStudio.Language.StandardClassification;
	
	using System.Windows.Forms;
	[Export(typeof(IClassifierProvider))]
    [ContentType("rust")]
    
    public sealed class RustClassifierProvider : IClassifierProvider
	{
		[Import]
		private IClassificationTypeRegistryService classificationRegistry;

		[Import]
		private IStandardClassificationService standardClassificationService;

		public IClassifier GetClassifier(ITextBuffer buffer)
		{
			MessageBox.Show("oh hi");

			return null;
			// return buffer.Properties.GetOrCreateSingletonProperty(creator: () => new RustClassifier(classificationRegistry, standardClassificationService));
		}
	}

}
