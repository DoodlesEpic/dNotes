gtknotes_sources = [
  'main.c',
  'gtknotes-window.c',
  'gtknotes-application.c',
]

gtknotes_deps = [
  dependency('gio-2.0', version: '>= 2.50'),
  dependency('gtk4', version: '>= 4.2'),
  dependency('gmodule-2.0', version: '>= 2.0'),
]

gnome = import('gnome')

gtknotes_sources += gnome.compile_resources('gtknotes-resources',
  'gtknotes.gresource.xml',
  c_name: 'gtknotes'
)

executable('gtknotes', gtknotes_sources,
  dependencies: gtknotes_deps,
  install: true,
)
