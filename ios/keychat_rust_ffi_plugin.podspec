#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint keychat_rust_ffi_plugin.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'keychat_rust_ffi_plugin'
  s.version          = '1.0.0'
  s.summary          = 'Keychat rust lib , FFI plugin project'
  s.description      = <<-DESC
A new Flutter FFI plugin project.
                       DESC
  s.homepage         = 'https://www.keychat.io/'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }

  # >>>>>> Everything after this line is new <<<<<<<

  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  
  s.ios.deployment_target  = '17.2'

  s.script_phase = {
    :name => 'Build Rust library',
    # First argument is relative path to the `rust` folder, second is name of rust library
    :script => 'sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" ../rust keychat_rust_ffi_plugin',
    :execution_position => :before_compile,
    :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
    # Let XCode know that the static library referenced in -force_load below is
    # created by this build step.
    :output_files => ["${BUILT_PRODUCTS_DIR}/libkeychat_rust_ffi_plugin.a"],
  }
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    'IPHONEOS_DEPLOYMENT_TARGET' => '17.2',
    # Flutter.framework does not contain a i386 slice.
    'EXCLUDED_ARCHS[sdk=iphone*]' => 'i386 x86_64',
    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libkeychat_rust_ffi_plugin.a',
  }
end