#ifndef TITLEBAR_H
#define TITLEBAR_H

#include <QWidget>
#include <QLabel>
#include <QPushButton>
#include <QHBoxLayout>
#include <QMainWindow>
#include <QDialog>
#include <QMessageBox>

class TitleBar : public QWidget
{
private:
    QWidget* parent;
    QPoint cursor;

    QHBoxLayout* layout;
    QLabel* icon;
    QLabel* title;
    QPushButton* minimiseButton;
    QPushButton* closeButton;
public:
    explicit TitleBar(QMainWindow* parent);

    void setWindowTitle(const QString& title);
    void setIconVisible(bool visible);
protected slots:
    void minimise();
protected:
    void mousePressEvent(QMouseEvent* event);
    void mouseMoveEvent(QMouseEvent* event);
};

#endif // TITLEBAR_H
