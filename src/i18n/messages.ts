export const messages: Record<string, Record<string, string>> = {
  en: {
    // App
    'app.title': 'Vert',
    'app.subtitle': 'Convert files between formats',
    'app.lang': '中文',

    // Drop zone
    'drop.title': 'Drop a file here',
    'drop.hint': 'or {link} to select a file',
    'drop.browse': 'browse',
    'drop.unknown': 'Unknown format',

    // Format selector
    'format.from': 'From',
    'format.to': 'To',
    'format.swap': 'Swap formats',
    'format.placeholder': 'Select a source format first',
    'format.none': 'No conversions available',

    // Convert button
    'convert.btn': 'Convert ↵',
    'convert.converting': 'Converting...',

    // Result
    'result.idle': 'Ready to convert',
    'result.converting': 'Converting',
    'result.processing': 'Processing your file...',
    'result.done': 'Done!',
    'result.converted': 'Converted in {duration}',
    'result.open': 'Open file',
    'result.showInFolder': 'Show in folder',
    'result.copyPath': 'Copy path',
    'result.copied': 'Copied!',
    'result.another': 'Convert another',
    'result.failed': 'Conversion failed',
    'result.tryAgain': 'Try again',
  },

  zh: {
    'app.title': 'Vert',
    'app.subtitle': '文件格式转换工具',
    'app.lang': 'English',

    'drop.title': '拖拽文件到此处',
    'drop.hint': '或{link}选择文件',
    'drop.browse': '点击',
    'drop.unknown': '未知格式',

    'format.from': '源格式',
    'format.to': '目标格式',
    'format.swap': '交换格式',
    'format.placeholder': '请先选择源格式',
    'format.none': '无可转换格式',

    'convert.btn': '转换 ↵',
    'convert.converting': '转换中...',

    'result.idle': '等待转换',
    'result.converting': '正在转换',
    'result.processing': '正在处理文件...',
    'result.done': '完成！',
    'result.converted': '耗时 {duration}',
    'result.open': '打开文件',
    'result.showInFolder': '打开所在文件夹',
    'result.copyPath': '复制路径',
    'result.copied': '已复制！',
    'result.another': '继续转换',
    'result.failed': '转换失败',
    'result.tryAgain': '重试',
  },
};
