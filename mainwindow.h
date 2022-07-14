#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_createNoteButton_clicked();

    void on_actionClear_triggered();
    void on_actionExit_triggered();

private:
    Ui::MainWindow *ui;
    bool saveFile(const QString &fileName, const QString text);
};
#endif // MAINWINDOW_H
