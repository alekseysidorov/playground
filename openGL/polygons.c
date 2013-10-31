#include "polygons.h"

#include <stdio.h>

#define drawOneLine(x1,y1,x2,y2) glBegin(GL_LINES); \
    glVertex2f((x1), (x2)); \
    glVertex2f((x2), (y2)); \
    glEnd()


#define X .525731112119133606
#define Z .850650808352039932

static GLfloat vdata[12][3] = {
    {-X, 0.0, Z}, {X, 0.0, Z}, {-X, 0.0, -Z}, {X, 0.0, -Z},
    {0.0, Z, X}, {0.0, Z, -X}, {0.0, -Z, X}, {0.0, -Z, -X},
    {Z, X, 0.0}, {-Z, X, 0.0}, {Z, -X, 0.0}, {-Z, -X, 0.0}
};
static GLuint tindicies[20][3] = {
    {1,4,0}, {4,9,0}, {4,5,9}, {8,5,4}, {1,8,4},
    {1,10,8}, {10,3,8}, {8,3,5}, {3,2,5}, {3,7,2},
    {3,10,7}, {10,6,7}, {6,11,7}, {6,0,11}, {6,1,0},
    {10,1,6}, {11,0,9}, {2,11,9}, {5,2,9}, {11,2,7}
};

void polygons_init(void)
{
    glClearColor(0.0, 0.0, 0.0, 0.0);
    glShadeModel(GL_FLOAT);
}

void polygons_display(void)
{
    glClear(GL_COLOR_BUFFER_BIT);

    glColor3f(1.0, 1.0, 1.0);

    glEnable(GL_POLYGON);
    glRectf(125.0, 25.0, 225.0, 125.0);
    glRectf(225.0, 25.0, 325.0, 125.0);
    glDisable(GL_POLYGON);

    glBegin(GL_TRIANGLES);
    glColor3f(1.0, 1.0, 1.0);
    for (int i = 0; i < 20; i++) {
        glVertex2fv(&vdata[tindicies[i][0]] [0]);
        glVertex2fv(&vdata[tindicies[i][1]] [0]);
        glVertex2fv(&vdata[tindicies[i][2]] [0]);
    }
    glEnd();

    glFlush();
}

void polygons_reshape(int w, int h)
{
    glViewport(0, 0, (GLsizei) w, (GLsizei) h);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();

    gluPerspective(45, (GLfloat) w / (GLfloat) h, 0.1, 200);
    //gluOrtho2D(0.0, (GLdouble) w, 0.0, (GLdouble) h);
}
