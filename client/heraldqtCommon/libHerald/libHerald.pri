
QT += quick svg xml
VERSION = 0.0.1

INCLUDEPATH += $$PWD/../../../libherald/qt_ffi

HEADERS += \
     $$PWD/../../../libherald/qt_ffi/Bindings.h

SOURCES += \
    $$PWD/../../../libherald/qt_ffi/Bindings.cpp


# set build type for Rust library
debug {
 RUST_BUILD_TYPE = debug
}
else {
  release | profile {
   RUST_BUILD_TYPE = release
  }
}

# untested, may need -lsqlite3
android {
    QMAKE_LFLAGS += -nostdlib++
    LIBS +=  $${PWD}/../../../libherald/target/i686-linux-android/$${RUST_BUILD_TYPE}/libherald.a
}

iphonesimulator {
    LIBS +=  $${PWD}/../../../libherald/target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
        -lsqlite3
}

macx {
  LIBS += -L $${PWD}/../../../libherald/target/$${RUST_BUILD_TYPE} -lherald
}

linux {
  LIBS += $${PWD}/../../../libherald/target/$${RUST_BUILD_TYPE}/libherald.so
}