using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using Microsoft.Build.Framework;
using Microsoft.Build.Utilities;
using VisualRust.Shared;
using Environment = VisualRust.Shared.Environment;

namespace VisualRust.Build
{
    public class Rustc : Task
    {
        private static readonly Regex DefectRegex = new Regex(@"^([^\n:]+):(\d+):(\d+):\s+(\d+):(\d+)\s+(.*)$", RegexOptions.Multiline | RegexOptions.CultureInvariant);

        private static readonly Regex ErrorCodeRegex = new Regex(@"\[([A-Z]\d\d\d\d)\]$", RegexOptions.CultureInvariant);

        /// <summary>
        /// Sets --cfg option.
        /// </summary>
        public string[] ConfigFlags { get; set; } = new string[0];

        /// <summary>
        /// Sets -L option.
        /// </summary>
        public string[] AdditionalLibPaths { get; set; } = new string[0];

        /// <summary>
        /// Sets --crate-type option.
        /// </summary>
        public string[] CrateType { get; set; } = new string[0];

        /// <summary>
        /// Sets --emit option.
        /// </summary>
        public string[] Emit { get; set; } = new string[0];

        /// <summary>
        /// Sets --crate-name option.
        /// </summary>
        public string CrateName { get; set; }

        private bool? _debugInfo;
        /// <summary>
        /// Sets -g option.
        /// </summary>
        public bool DebugInfo 
        {
            get { return _debugInfo.HasValue && _debugInfo.Value; }
            set { _debugInfo = value; }
        }

        /// <summary>
        /// Sets -o option.
        /// </summary>
        public string OutputFile { get; set; }

        private int? _optimizationLevel;
        /// <summary>
        /// Sets --opt-level option. Default value is 0.
        /// </summary>
        public int OptimizationLevel
        {
            get { return _optimizationLevel ?? 0; }
            set { _optimizationLevel = value; }
        }

        /// <summary>
        /// Sets --out-dir option.
        /// </summary>
        public string OutputDirectory { get; set; }

        private bool? _test;
        /// <summary>
        /// Sets --test option. Default value is false.
        /// </summary>
        public bool Test
        {
            get { return _test ?? false; }
            set { _test = value; }
        }

        /// <summary>
        /// Sets --target option.
        /// </summary>
        public string TargetTriple { get; set; }


        /// <summary>
        /// Sets -W option.
        /// </summary>
        public string[] LintsAsWarnings { get; set; } = new string[0];

        /// <summary>
        /// Sets -A option.
        /// </summary>
        public string[] LintsAsAllowed { get; set; } = new string[0];

        /// <summary>
        /// Sets -D option.
        /// </summary>
        public string[] LintsAsDenied { get; set; } = new string[0];

        /// <summary>
        /// Sets -F option.
        /// </summary>
        public string[] LintsAsForbidden { get; set; } = new string[0];

        /// <summary>
        /// Sets -C option.
        /// </summary>
        public string CodegenOptions { get; set; }

        private bool? _lto;
        /// <summary>
        /// Sets -C lto option. Default value is false.
        /// </summary>
        public bool Lto
        {
            get { return _lto ?? false; }
            set { _lto = value; }
        }

        [Required]
        public string WorkingDirectory { get; set; }

        [Required]
        public string Input { get; set; }

        public string AdditionalRustcOptions { get; set; } = string.Empty;

        public override bool Execute()
        {
            try
            {
                return ExecuteInner();
            }
            catch (Exception ex)
            {
                Log.LogErrorFromException(ex, true);
                return false;
            }
        }

        private bool ExecuteInner()
        {
            var sb = new StringBuilder();
            if (ConfigFlags.Length > 0)
                sb.AppendFormat(" --cfg {0}", string.Join(",", ConfigFlags));
            if (AdditionalLibPaths.Length > 0)
                sb.AppendFormat(" -L {0}", string.Join(",", AdditionalLibPaths));
            if(CrateType.Length > 0)
                sb.AppendFormat(" --crate-type {0}", string.Join(",",CrateType));
            if(Emit.Length > 0)
                sb.AppendFormat(" --emit {0}", string.Join(",", Emit));
            if(!string.IsNullOrWhiteSpace(CrateName))
                sb.AppendFormat(" --crate-name {0}", CrateName);
            if(DebugInfo)
                sb.AppendFormat(" -g");
            if(OutputFile != null)
                sb.AppendFormat(" -o {0}", OutputFile);
            if (_optimizationLevel.HasValue)
                sb.AppendFormat(" -C opt-level={0}", OptimizationLevelExtension.Parse(OptimizationLevel.ToString()).ToBuildString());
            if (OutputDirectory != null)
                sb.AppendFormat(" --out-dir {0}", OutputDirectory);
            if (_test.HasValue && _test.Value)
                sb.Append(" --test");
            if (TargetTriple != null && !string.Equals(TargetTriple, Environment.DefaultTarget, StringComparison.OrdinalIgnoreCase))
                sb.AppendFormat(" --target {0}", TargetTriple);
            if(LintsAsWarnings.Length > 0)
                sb.AppendFormat(" -W {0}", string.Join(",", LintsAsWarnings));
            if(LintsAsAllowed.Length > 0)
                sb.AppendFormat(" -A {0}", string.Join(",", LintsAsAllowed));
            if(LintsAsDenied.Length > 0)
                sb.AppendFormat(" -D {0}", string.Join(",", LintsAsDenied));
            if(LintsAsForbidden.Length > 0)
                sb.AppendFormat(" -F {0}", string.Join(",", LintsAsForbidden));
            if (_lto.HasValue && _lto.Value)
                sb.AppendFormat(" -C lto");

            if (CodegenOptions != null)
                sb.Append($" -C {CodegenOptions}");

            if (!string.IsNullOrWhiteSpace(AdditionalRustcOptions))
                sb.Append($" {AdditionalRustcOptions}");

            sb.AppendFormat(" {0}", Input);
            var target = TargetTriple ?? Environment.DefaultTarget;
            var installPath = Environment.FindInstallPath(target);
            if(installPath == null)
            {
                if(string.Equals(target, Environment.DefaultTarget, StringComparison.OrdinalIgnoreCase))
                    Log.LogError("No Rust installation detected. You can download official Rust installer from https://www.rust-lang.org/downloads.html");
                else
                    Log.LogError("Could not find a Rust installation that can compile target {0}.", target);
                return false;
            }
            var psi = new ProcessStartInfo
            {
                CreateNoWindow = true,
                FileName =  Path.Combine(installPath, "rustc.exe"),
                UseShellExecute = false,
                WorkingDirectory = WorkingDirectory,
                Arguments = sb.ToString(),
                RedirectStandardError = true
            };
            Log.LogCommandLine(string.Join(" ", psi.FileName, psi.Arguments));
            try
            {
                var process = new Process {StartInfo = psi};
                var error = new StringBuilder();

                using (var errorWaitHandle = new AutoResetEvent(false))
                {
                    process.ErrorDataReceived += (sender, e) =>
                    {
                        if (e.Data == null)
                        {
                            errorWaitHandle.Set();
                        }
                        else
                        {
                            error.AppendLine(e.Data);
                        }
                    };

                    process.Start();
                    process.BeginErrorReadLine();
                    process.WaitForExit();
                    errorWaitHandle.WaitOne();
                }

                var errorOutput = error.ToString();
                // We found some warning or errors in the output, print them out
                var messages = ParseOutput(errorOutput);
                // We found some warning or errors in the output, print them out
                foreach (var msg in messages)
                {
                    LogRustcMessage(msg);
                }
                // rustc failed but we couldn't sniff anything from stderr
                // this could be an internal compiler error or a missing main() function (there are probably more errors without spans)
                if (process.ExitCode != 0 && !messages.Any())
                {
                    // FIXME: This automatically sets the file to VisualRust.Rust.targets. Is there a way to set no file instead?
                    Log.LogError(errorOutput);
                    return false;
                }
                return process.ExitCode == 0;
            }
            catch(Exception ex)
            {
                Log.LogErrorFromException(ex, true);
                return false;
            }
        }
        
        private static IEnumerable<RustcParsedMessage> ParseOutput(string output)
        {
            var errorMatches = DefectRegex.Matches(output);

            RustcParsedMessage previous = null;
            foreach (Match match in errorMatches)
            {
                var remainingMsg = match.Groups[6].Value.Trim();
                var errorMatch = ErrorCodeRegex.Match(remainingMsg);
                var errorCode = errorMatch.Success ? errorMatch.Groups[1].Value : null;
                var line = int.Parse(match.Groups[2].Value, NumberStyles.None);
                var col = int.Parse(match.Groups[3].Value, NumberStyles.None);
                var endLine = int.Parse(match.Groups[4].Value, NumberStyles.None);
                var endCol = int.Parse(match.Groups[5].Value, NumberStyles.None);

                if (remainingMsg.StartsWith("warning: "))
                {
                    var msg = match.Groups[6].Value.Substring(9, match.Groups[6].Value.Length - 9 - (errorCode != null ? 8 : 0));
                    if (previous != null) yield return previous;
                    previous = new RustcParsedMessage(RustcParsedMessageType.Warning, msg, errorCode, match.Groups[1].Value,
                        line, col, endLine, endCol);
                }
                else if (remainingMsg.StartsWith("note: ") || remainingMsg.StartsWith("help: "))
                {
                    if (remainingMsg.StartsWith("help: pass `--explain ") && previous != null)
                    {
                        previous.CanExplain = true;
                        continue;
                    }

                    // NOTE: "note: " and "help: " are both 6 characters long (though hardcoding this is probably still not a very good idea)
                    var msg = remainingMsg.Substring(6, remainingMsg.Length - 6 - (errorCode != null ? 8 : 0));
                    var type = remainingMsg.StartsWith("note: ") ? RustcParsedMessageType.Note : RustcParsedMessageType.Help;
                    var note = new RustcParsedMessage(type, msg, errorCode, match.Groups[1].Value,
                        line, col, endLine, endCol);

                    if (previous != null)
                    {
                        // try to merge notes and help messages with a previous message (warning or error where it belongs to), if the span is the same
                        if (previous.TryMergeWithFollowing(note))
                        {
                            continue; // skip setting new previous, because we successfully merged the new note into the previous message
                        }
                        yield return previous;
                    }
                    previous = note;
                }
                else
                {
                    var startsWithError = remainingMsg.StartsWith("error: ");
                    var msg = remainingMsg.Substring((startsWithError ? 7 : 0), remainingMsg.Length - (startsWithError ? 7 : 0) - (errorCode != null ? 8 : 0));
                    if (previous != null) yield return previous;
                    previous = new RustcParsedMessage(RustcParsedMessageType.Error, msg, errorCode, match.Groups[1].Value,
                        line, col, endLine, endCol);
                }
            }

            if (previous != null) yield return previous;
        }

        private void LogRustcMessage(RustcParsedMessage msg)
        {
            if (msg.Type == RustcParsedMessageType.Warning)
            {
                Log.LogWarning(null, msg.ErrorCode, null, msg.File, msg.LineNumber, msg.ColumnNumber, msg.EndLineNumber, msg.EndColumnNumber, msg.Message);
            }
            else if (msg.Type == RustcParsedMessageType.Note)
            {
                Log.LogWarning(null, msg.ErrorCode, null, msg.File, msg.LineNumber, msg.ColumnNumber, msg.EndLineNumber, msg.EndColumnNumber, "note: " + msg.Message);
            }
            else
            {
                Log.LogError(null, msg.ErrorCode, null, msg.File, msg.LineNumber, msg.ColumnNumber, msg.EndLineNumber, msg.EndColumnNumber, msg.Message);
            }
        }
    }
}
