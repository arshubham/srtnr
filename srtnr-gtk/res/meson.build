icon_sizes = ['16','24','32', '48', '64', '128']

foreach i : icon_sizes
    install_data(
        join_paths('images', 'icons', i, meson.project_name() + '.svg'),
        install_dir: join_paths(get_option('datadir'), 'icons', 'hicolor', i + 'x' + i, 'apps')
    )
endforeach

install_data('com.github.arshubham.srtnr.desktop', install_dir : datadir + '/applications')
install_data('com.github.arshubham.srtnr.appdata.xml', install_dir : datadir + '/appdata')
