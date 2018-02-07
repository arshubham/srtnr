project(
  'com.github.arshubham.srtnr', 'rust',
  version: '0.1.5',
  license: 'GPLv3',
)

srtnr_version = meson.project_version()
version_array = srtnr_version.split('.')
srtnr_major_version = version_array[0].to_int()
srtnr_minor_version = version_array[1].to_int()
srtnr_version_micro = version_array[2].to_int()

srtnr_prefix = get_option('prefix')
srtnr_bindir = join_paths(srtnr_prefix, get_option('bindir'))

datadir = get_option('datadir')
icondir = join_paths(datadir, 'icons')

subdir('data')

#Add path for rustup here later
cargo = find_program('cargo', required: false)
gresource = find_program('glib-compile-resources', required: false)
cargo_script = find_program('scripts/cargo.sh')
meson.add_install_script('meson/post_install.py')

cargo_release = custom_target('cargo-build',
                        build_by_default: true,
                        build_always: true,
                        output: ['com.github.arshubham.srtnr'],
                        install: true,
                        install_dir: srtnr_bindir,
                        command: [cargo_script, '@CURRENT_SOURCE_DIR@', '@OUTPUT@'])
