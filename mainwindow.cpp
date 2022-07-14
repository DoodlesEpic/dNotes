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
        qWarning() << "Text: " << noteText;
        return;
    }

    qDebug() << "Note created";
    QTextStream out(&file);
    out << noteText;
}

