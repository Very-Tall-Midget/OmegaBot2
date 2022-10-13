#include "titlebar.h"

#include <QMouseEvent>
#include <QFont>

TitleBar::TitleBar(QMainWindow* parent)
    : parent(parent)
{
    icon = new QLabel;
    icon->setObjectName("icon");

    title = new QLabel(parent->windowTitle());
    title->setObjectName("title");

    minimiseButton = new QPushButton("—");
    closeButton = new QPushButton("✕");
    minimiseButton->setCursor(Qt::PointingHandCursor);
    closeButton->setCursor(Qt::PointingHandCursor);
    minimiseButton->setObjectName("minimiseButton");
    closeButton->setObjectName("closeButton");

    layout = new QHBoxLayout(this);
    layout->addWidget(icon);
    layout->addWidget(title);
    layout->addWidget(minimiseButton);
    layout->addWidget(closeButton);

    connect(minimiseButton, SIGNAL(clicked()), parent, SLOT(showMinimized()));
    connect(closeButton, SIGNAL(clicked()), parent, SLOT(close()));
}

void TitleBar::setWindowTitle(const QString& title)
{
    this->title->setText(title);
}

void TitleBar::setIconVisible(bool visible)
{
    icon->setVisible(visible);
}

void TitleBar::mousePressEvent(QMouseEvent* event)
{
    if(event->button() == Qt::LeftButton)
    {
        cursor = event->globalPos() - parent->geometry().topLeft();
        event->accept();
    }
}

void TitleBar::mouseMoveEvent(QMouseEvent *event)
{
    if(event->buttons() & Qt::LeftButton)
    {
        parent->move(event->globalPos() - cursor);
        event->accept();
    }
}
