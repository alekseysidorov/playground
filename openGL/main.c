#include <stdlib.h>

#include "lines.h"
#include "polygons.h"

int main(int argc, char **argv)
{
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB);
    glutInitWindowSize(1024, 768);
    glutInitWindowPosition(100, 100);
    glutCreateWindow(argv[0]);

    polygons_init();
    glutDisplayFunc(polygons_display);
    glutReshapeFunc(polygons_reshape);
    glutIdleFunc(polygons_idle);

    glutMainLoop();
    return 0;
}

