import qbs

CppApplication {
    property string sanitizer: ""

    files: [
        "blinker.c",
        "blinker.h",
        "ledmodel.cpp",
        "ledmodel.h",
        "main.cpp",
    ]
    cpp.cxxLanguageVersion: "c++14"

    Group {     // Properties for the produced executable
        fileTagsFilter: product.type
        qbs.install: true
        qbs.installDir: "bin"
    }

    Depends { name: "Qt.quick" }
    Depends { name: "Qt.widgets" }

    Group {
        name: "qml"

        qbs.install: true
        qbs.installDir: "bin/qml/"

        prefix: "qml/"
        files: [
            "*.qml",
        ]
    }

    Properties {
        condition: sanitizer === "address"

        cpp.commonCompilerFlags: "-fsanitize=address"
        cpp.linkerFlags: "-fsanitize=address"
    }
}
