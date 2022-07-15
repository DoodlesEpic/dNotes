gtknotes_sources = [
  'main.c',
  'gtknotes-window.c',
  'gtknotes-application.c',
]

gtknotes_deps = [
  dependency('libadwaita-1', version: '>= 1.0'),
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
