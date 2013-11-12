#include "polygons.h"

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>

static float lol = 1.0;
static float distance = 0;
static float distanceInc = 0.02;

double rand_my()
{
    double v = ((double) rand() / (RAND_MAX));
    return v;
}

void polygons_init(void)
{
    glClearColor(0.0, 0.0, 0.0, 0.0);
    glShadeModel(GL_FLOAT);
}

void normalize(GLfloat v[3])
{
    float d = 0;
    for (int i = 0; i < 3; i++)
        d += v[i]*v[i];
    d = sqrt(d);
    for (int i = 0; i < 3; i++)
        v[i] /= d;
}

void dotCrossProduct(GLfloat a[3], GLfloat b[3], GLfloat out[3])
{
    out[0] = a[1] * b[2] - a[2] * b[1];
    out[1] = a[2] * b[0] - a[0] * b[2];
    out[2] = a[0] * b[1] - a[1] * b[0];
}

void normalToTriangle(GLfloat vA[3], GLfloat vB[3], GLfloat vC[3], GLfloat out[3])
{
    GLfloat v1[3] = { vA[0] - vC[0], vA[1] - vC[1], vA[2] - vC[2] };
    GLfloat v2[3] = { vB[0] - vC[0], vB[1] - vB[1], vB[2] - vC[2] };

    dotCrossProduct(v1, v2, out);
    normalize(out);
}

void setNormal(GLfloat vA[3], GLfloat vB[3], GLfloat vC[3])
{
    GLfloat normal[3];
    normalToTriangle(vA, vB, vC, normal);
    glNormal3f(normal[0], normal[1], normal[2]);
}

void drawTriangle3f(GLfloat vA[3], GLfloat vB[3], GLfloat vC[3], int deep)
{
    GLfloat normal[3];

    normalToTriangle(vA, vB, vC, normal);
    GLfloat center[3] = {
        (vA[0] + vB[0] + vC[0]) / 3,
        (vA[1] + vB[1] + vC[1]) / 3,
        (vA[2] + vB[2] + vC[2]) / 3,
    };

    double f = 1.22474487139 ; ///compute it
    GLfloat vD[3] = {
        center[0] + normal[0] * f,
        center[1] + normal[1] * f,
        center[2] + normal[2] * f,
    };

    //if (deep >= 1) {
    //    //glBegin(GL_TRIANGLE_STRIP);
    //    //glColor3f(1.0, 1.0, 1.0);
    //    //glVertex3fv(vA);
    //    //glVertex3fv(vB);
    //    //glVertex3fv(vC);
    //    //glColor3f(0.0, 0.0, 1.0);
    //    //glVertex3fv(vD);
    //    //glColor3f(1.0, 0.0, 0.0);
    //    //glVertex3fv(vB);
    //    //glColor3f(0.0, 1.0, 0.0);
    //    //glVertex3fv(vA);
    //    //glEnd();
    //} else {
    //    deep++;

    //    drawTriangle3f(vA, vB, vD, deep);
    //    drawTriangle3f(vA, vC, vD, deep);
    //    drawTriangle3f(vB, vC, vD, deep);
    //}

    if (deep < 0) {
        deep++;
        drawTriangle3f(vB, vA, vC, deep);
        //drawTriangle3f(vA, vB, vD, deep);
        //drawTriangle3f(vC, vB, vD, deep);
        //drawTriangle3f(vC, vA, vD, deep);
    } //else {
        glBegin(GL_TRIANGLE_STRIP);
        glColor3f(1.0, 1.0, 1.0);
        glVertex3fv(vA); //0
        glVertex3fv(vB); //1
        glVertex3fv(vC); //2
        glNormal3fv(normal);
        glColor3f(0.0, 0.0, 1.0);
        glVertex3fv(vD); //3
        setNormal(vB, vC, vD);
        glColor3f(1.0, 0.0, 0.0);
        glVertex3fv(vA); //0
        setNormal(vC, vD, vA);
        glColor3f(0.0, 1.0, 0.0);
        glVertex3fv(vB); //1
        setNormal(vD, vB, vA);
        glEnd();
    //}
}

void polygons_display(void)
{
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3f(1.0, 0.0, 1.0);
    glLoadIdentity();

    glTranslatef(0.0, 0.0, -5 + distance);
    glRotatef(lol, 0.3, 0.1, 0.5);
    glScalef(2, 2, 2);

    //Vertices below are in Clockwise orientation
    //Default setting for glFrontFace is Counter-clockwise
    glFrontFace(GL_CW);

    GLfloat v[3][3] = {
        {-1.0f, 0.0f, 0.0f},
        {1.0f, 0.0f, 0.0f},
        {0.0f, -1.0f, 0.0f}
    };
    //glEnable(GL_CULL_FACE);
    //glCullFace(GL_FRONT_AND_BACK);
    drawTriangle3f(v[0], v[1], v[2], 0);

    //glBegin(GL_TRIANGLE_FAN);

    //glVertex3f(-1.0f, -0.5f, -2.0f);    // A
    //glColor3f(0.3, 1.0, 0.0);
    //glVertex3f( 1.0f, -0.5f, -4.0f);    // B
    //glColor3f(1.0, 0, 1.0);
    //glVertex3f( 0.0f,  0.5f, -4.0f);    // C
    //glColor3f(1.0, 0.5, 0.0);

    //glVertex3f(-1.5f,  0.0f, -4.0f);    // D
    //glColor3f(0.7, 0.0, 0.0);
    //glVertex3f(-1.8f, -1.0f, -4.0f);    // E
    //glColor3f(0.5, 0.0, 1.0);
    //glVertex3f( 0.2f, -1.5f, -4.0f);    // F

    //glVertex3f( 1.0f, -0.5f, -4.0f);    // G

    //glEnd();

    glutSwapBuffers();
}

void polygons_idle()
{
    if (distance >= 2 || distance <= -2.0)
        distanceInc *= -1;
    distance += distanceInc;
    lol += 0.4;
    glutPostRedisplay();
}

void polygons_reshape(int w, int h)
{
    glViewport(0, 0, (GLsizei) w, (GLsizei) h);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glFrustum(-1.0, 1.0, -1.0, 1.0, 1, 80);
    glMatrixMode(GL_MODELVIEW);
}
