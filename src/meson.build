pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')

blueprints = custom_target('blueprints',
  input: files (
    'gtk/discover-view.blp',
    'gtk/empty-view.blp',
    'gtk/discover-card.blp',
    'gtk/help-overlay.blp',
    'gtk/podcasts-view-stack.blp',
    'gtk/queue-view.blp',
    'gtk/window.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
      'batch-compile',
      '@OUTPUT@',
      '@CURRENT_SOURCE_DIR@',
      '@INPUT@'
    ]
)

gnome.compile_resources('bolt',
  'bolt.gresource.xml',
  gresource_bundle: true,
  dependencies: blueprints,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'bolt')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('USER_AGENT', 'Bolt/1.0')
conf.set_quoted('BASE_URL', 'https://api.podcastindex.org/api/1.0')
conf.set_quoted('API_KEY', '')
conf.set_quoted('API_SECRET', '')

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)

cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env', cargo_env,
    cargo_bin, 'build',
    cargo_opt, '&&', 'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
