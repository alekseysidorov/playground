#include <stdlib.h>

#include "lines.h"
#include "polygons.h"

int main(int argc, char **argv)
{
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowSize(400, 150);
    glutInitWindowPosition(100, 100);
    glutCreateWindow(argv[0]);

    polygons_init();
    glutDisplayFunc(polygons_display);
    glutReshapeFunc(polygons_reshape);

    glutMainLoop();
    return 0;
}

