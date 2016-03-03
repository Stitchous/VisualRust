using Microsoft.VisualStudio.Text.Tagging;
using Microsoft.VisualStudio.Utilities;
using System.ComponentModel.Composition;
using Microsoft.VisualStudio.Text;
using System;
using VisualRust.Grammar;

namespace VisualRust
{
	[Export(typeof(ITaggerProvider))]
	[TagType(typeof(IOutliningRegionTag))]
	[ContentType("rust")]
	internal sealed class RustOutliningTaggerProvider : ITaggerProvider
	{
		public ITagger<T> CreateTagger<T>(ITextBuffer buffer) where T : ITag
		{
			return buffer.Properties.GetOrCreateSingletonProperty<ITagger<T>>(
					() => new OutliningTagger(buffer) as ITagger<T>);
		}

		
	}
}
