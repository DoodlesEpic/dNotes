#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QFile>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    this->setCentralWidget(ui->verticalLayoutWidget);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::on_createNoteButton_clicked()
{
    const QString noteText= ui->textEdit->toPlainText();
    qDebug() << "Text: " << noteText;

    QFile file("note.txt");
    if (!file.open(QIODevice::WriteOnly | QIODevice::Text)) {
        qWarning() << "Failed to open file";
        return;
    }

    qDebug() << "Note created";
    QTextStream out(&file);
    out << noteText;
}


void MainWindow::on_actionClear_triggered()
{
    ui->textEdit->clear();
    qDebug() << "Note clear";
}


void MainWindow::on_actionExit_triggered()
{
    qDebug() << "Leaving...";
    qApp->exit();
}

