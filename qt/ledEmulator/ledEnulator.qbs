import qbs

CppApplication {
    files: [
        "main.cpp"
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
        files: [ "*.qml" ]
    }
}
