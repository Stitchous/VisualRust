﻿using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using VisualRust.Cargo;

namespace VisualRust.Project.Controls
{
    public partial class OutputPage : DockPanel
    {
        public OutputPage()
        {
            InitializeComponent();
        }
    }

    public class OutputTargetGroupDescription : GroupDescription
    {
        public override object GroupNameFromItem(object item, int level, CultureInfo culture)
        {
            IOutputTargetViewModel vm = item as IOutputTargetViewModel;
            if (vm == null)
                return null;
            if(item is CommandOutputTargetViewModel)
                return "Custom targets";
            if (vm.IsAutoGenerated)
                return "Default targets";
            else
                return "Custom targets";
        }
    }

    public class ItemTemplateSelector : DataTemplateSelector
    {
        public DataTemplate Benchmark { get; set; }
        public DataTemplate Binary { get; set; }
        public DataTemplate Example { get; set; }
        public DataTemplate Library { get; set; }
        public DataTemplate Test { get; set; }
        public DataTemplate Add { get; set; }

        public override DataTemplate SelectTemplate(object item, DependencyObject container)
        {
            IOutputTargetViewModel vm = item as IOutputTargetViewModel;
            if (item == null)
                return base.SelectTemplate(item, container);
            if(item is CommandOutputTargetViewModel)
                return Add;
            switch (vm.Type)
            {
                case OutputTargetType.Benchmark:
                    return Benchmark;
                case OutputTargetType.Binary:
                    return Binary;
                case OutputTargetType.Example:
                    return Example;
                case OutputTargetType.Library:
                    return Library;
                case OutputTargetType.Test:
                    return Test;
            }
            return base.SelectTemplate(item, container);
        }
    }

    public class ContentTemplateSelector : DataTemplateSelector
    {
        public DataTemplate Custom { get; set; }
        public DataTemplate Auto { get; set; }

        public override DataTemplate SelectTemplate(object item, DependencyObject container)
        {
            IOutputTargetViewModel vm = item as IOutputTargetViewModel;
            if (item == null)
                return base.SelectTemplate(item, container);
            if(vm.IsAutoGenerated)
                return Auto;
            return Custom;
        }
    }

    class ItemContainerStyleSelector : StyleSelector
    {
        public Style Default { get; set; }
        public Style Button { get; set; }

        public override Style SelectStyle(object item, DependencyObject container)
        {
            IOutputTargetViewModel vm = item as IOutputTargetViewModel;
            if (item == null)
                return base.SelectStyle(item, container);
            if(item is CommandOutputTargetViewModel)
                return Button;
            return Default;
        }
    }

    class InvisibilityConverter : IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
        {
            bool invisible = false;
            if (value is bool)
                invisible = (bool)value;
            return invisible ? Visibility.Collapsed : Visibility.Visible;
        }

        public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture)
        {
            if (value is Visibility)
                return (Visibility)value == Visibility.Collapsed;
            return false;
        }
    }

    class CollapseNullConverter : IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
        {
            return value == null ? Visibility.Collapsed : Visibility.Visible;
        }

        public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture)
        {
            return DependencyProperty.UnsetValue;
        }
    }

    class TargetIconConverter : IValueConverter
    {
        public DrawingGroup Benchmark { get; set; }
        public DrawingGroup Binary { get; set; }
        public DrawingGroup Example { get; set; }
        public DrawingGroup Library { get; set; }
        public DrawingGroup Test { get; set; }

        public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
        {
            if(value is OutputTargetType)
            {
                switch((OutputTargetType)value)
                {
                    case OutputTargetType.Benchmark:
                        return Benchmark;
                    case OutputTargetType.Binary:
                        return Binary;
                    case OutputTargetType.Example:
                        return Example;
                    case OutputTargetType.Library:
                        return Library;
                    case OutputTargetType.Test:
                        return Test;
                }
            }
            return DependencyProperty.UnsetValue;
        }

        public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture)
        {
            return DependencyProperty.UnsetValue;
        }
    }
}
