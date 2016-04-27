namespace VisualRust.Build
{
    internal enum RustcParsedMessageType
    {
        Error,
        Warning,
        Note,
        Help
    }

    internal class RustcParsedMessage
    {
        public RustcParsedMessageType Type;
        public string Message;
        public string ErrorCode;
        public string File;
        public int LineNumber;
        public int ColumnNumber;
        public int EndLineNumber;
        public int EndColumnNumber;
        public bool CanExplain; // TODO: currently we don't do anything with this

        public RustcParsedMessage(RustcParsedMessageType type, string message, string errorCode, string file,
            int lineNumber, int columnNumber, int endLineNumber, int endColumnNumber)
        {
            Type = type;
            Message = message;
            ErrorCode = errorCode;
            File = file;
            LineNumber = lineNumber;
            ColumnNumber = columnNumber;
            EndLineNumber = endLineNumber;
            EndColumnNumber = endColumnNumber;
            CanExplain = false;
        }

        public bool TryMergeWithFollowing(RustcParsedMessage other)
        {
            if ((other.Type == RustcParsedMessageType.Note || other.Type == RustcParsedMessageType.Help)
                && other.File == File && other.LineNumber == LineNumber && other.ColumnNumber == ColumnNumber &&
                other.EndLineNumber == EndLineNumber && other.EndColumnNumber == EndColumnNumber)
            {
                var prefix = other.Type == RustcParsedMessageType.Note ? "\nnote: " : "\nhelp: ";
                Message += prefix + other.Message;
                return true;
            }
            return false;
        }
    }
}
