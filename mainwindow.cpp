#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QFile>
#include <QFileDialog>
#include <QSaveFile>
#include <QMessageBox>

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

    QFileDialog dialog(this);
    dialog.setWindowModality(Qt::WindowModal);
    dialog.setAcceptMode(QFileDialog::AcceptSave);
    if (dialog.exec() != QDialog::Accepted) {
        qWarning() << "Save note cancelled";
        return;
    }

    saveFile(dialog.selectedFiles().first(), noteText);
}

bool MainWindow::saveFile(const QString &fileName, const QString text)
{
    QString errorMessage;

    QGuiApplication::setOverrideCursor(Qt::WaitCursor);
    QSaveFile file(fileName);
    if (file.open(QFile::WriteOnly | QFile::Text))
    {
        QTextStream out(&file);
        out << text;
        if (!file.commit())
        {
            errorMessage = tr("Cannot write file %1:\n%2.")
                           .arg(QDir::toNativeSeparators(fileName), file.errorString());
        }
    }
    else
    {
        errorMessage = tr("Cannot open file %1 for writing:\n%2.")
                       .arg(QDir::toNativeSeparators(fileName), file.errorString());
    }
    QGuiApplication::restoreOverrideCursor();

    if (!errorMessage.isEmpty())
    {
        QMessageBox::warning(this, tr("Application"), errorMessage);
        return false;
    }

    statusBar()->showMessage(tr("File saved"), 2000);
    return true;
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

