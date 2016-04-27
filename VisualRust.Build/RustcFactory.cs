using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using Microsoft.Build.Framework;

namespace VisualRust.Build
{
    public class RustcFactory : ITaskFactory
    {
        private TaskPropertyInfo[] _parameters;

        public void CleanupTask(ITask task)
        {
        }

        public ITask CreateTask(IBuildEngine taskFactoryLoggingHost)
        {
            return new Rustc();
        }

        public string FactoryName => "VisualRust.Build.RustcFactory";

        public TaskPropertyInfo[] GetTaskParameters()
        {
            _parameters = _parameters ?? typeof(Rustc)
                .GetProperties(BindingFlags.Instance | BindingFlags.Public)
                .Select(prop => 
                    new TaskPropertyInfo(
                        prop.Name,
                        prop.PropertyType,
                        prop.GetCustomAttributes().OfType<OutputAttribute>().Any(),
                        prop.GetCustomAttributes().OfType<RequiredAttribute>().Any()))
                .ToArray();
            return _parameters;
        }

        public bool Initialize(string taskName, IDictionary<string, TaskPropertyInfo> parameterGroup, string taskBody, IBuildEngine taskFactoryLoggingHost)
        {
            return true;
        }

        public Type TaskType => typeof(Rustc);
    }
}
