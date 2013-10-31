import qbs

CppApplication {
    type: "application" // To suppress bundle generation on Mac
    files: [ "*.c", "*.cpp", "*.h" ]

    Properties {
        condition: qbs.targetOS.contains("windows")
        cpp.dynamicLibraries: base.concat("opengl32")
    }
    Properties {
        condition: qbs.targetOS.contains("linux")
        cpp.dynamicLibraries: base.concat("GL")
    }
    Properties {
        condition: qbs.targetOS.contains("osx")
        cpp.frameworks: base.concat(["OpenGL", "GLUT"])
    }
}

