project('advent-of-code-2024', 'c',
        meson_version : '>=0.50.0',
        version: '0.1.0',
        license: 'MIT',
        default_options : ['c_std=c11', 'warning_level=2']
       )

prefix = get_option('prefix')
localedir = get_option('localedir')

cc = meson.get_compiler('c')
version = 'v' + meson.project_version()

add_project_arguments(
    cc.get_supported_arguments([
      '-Wno-unused-parameter',
      '-Wno-unused-value',
    ]),
    '-D_GNU_SOURCE',
    language: 'c'
)

config = configuration_data()
config.set('package', meson.project_name())
config.set('version', version)

deps = []

# Math library
libm = cc.find_library('m', required: true)
deps += libm

sources = files('include/quicksort.c',
                # 'day-1/historian-hysteria.c',
                # 'day-2/red-nosed-reports.c',
                'day-9/disk-fragmenter.c',
               ) # TODO: Expand this later manually
executable(meson.project_name(), sources, dependencies: deps)
