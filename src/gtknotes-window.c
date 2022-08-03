/* gtknotes-window.c
 *
 * Copyright 2022 DoodlesEpic
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "gtknotes-window.h"
#include "gtknotes-config.h"

struct _GtknotesWindow {
  GtkApplicationWindow parent_instance;
};

G_DEFINE_TYPE(GtknotesWindow, gtknotes_window, GTK_TYPE_APPLICATION_WINDOW)

GtkTextBuffer *note_buffer;
GtkFileChooserNative *native;
GtkFileChooser *chooser;

static void gtknotes_window_class_init(GtknotesWindowClass *klass) {
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);

  gtk_widget_class_set_template_from_resource(
      widget_class, "/dev/doodles/gtknotes/gtknotes-window.ui");
}

static void gtknotes_window_init(GtknotesWindow *self) {
  gtk_widget_init_template(GTK_WIDGET(self));

  // Create the native dialog to be shown when saving the note
  GtkApplicationWindow parent_instance = self->parent_instance;
  native = gtk_file_chooser_native_new(
      "Save File", GTK_WINDOW(&parent_instance), GTK_FILE_CHOOSER_ACTION_SAVE,
      "_Save", "_Cancel");
  chooser = GTK_FILE_CHOOSER(native);
  g_signal_connect(native, "response", G_CALLBACK(on_response), NULL);
}

void handle_note_text_changed(GtkTextBuffer *buffer) { note_buffer = buffer; list_notes(); }

void handle_create_note(GtkButton *b) {
  GtkTextIter start, end;
  gtk_text_buffer_get_start_iter(note_buffer, &start);
  gtk_text_buffer_get_end_iter(note_buffer, &end);

  g_print("%s\n", gtk_text_buffer_get_text(note_buffer, &start, &end, FALSE));

  // Show the native file dialog we created on window init
  gtk_file_chooser_set_current_name(chooser, "note.txt");
  gtk_native_dialog_show(GTK_NATIVE_DIALOG(native));
}

void on_response(GtkNativeDialog *native, int response) {
  g_assert(GTK_IS_NATIVE_DIALOG (native));

  if (response == GTK_RESPONSE_ACCEPT) {
    chooser = GTK_FILE_CHOOSER(native);
    g_autoptr(GFile) file = gtk_file_chooser_get_file(chooser);
    save_to_file(file);
  }
}

void save_to_file(GFile *file) {
  g_assert(G_IS_FILE(file));

  GtkTextIter start, end;
  gtk_text_buffer_get_start_iter(note_buffer, &start);
  gtk_text_buffer_get_end_iter(note_buffer, &end);

  g_file_set_contents(
      g_file_get_path(file),
      gtk_text_buffer_get_text(note_buffer, &start, &end, FALSE),
      gtk_text_buffer_get_char_count(note_buffer), NULL);
}

// Attempt to print number of files on the data dir
// Currently only gives 0 for some reason. Confirmed to not be caused by flatpak
// permissions.
// Currently this is called in every change to the text field for testing.
void list_notes() {
  g_autoptr(GFile) directory = g_file_new_build_filename(g_get_user_data_dir(), NULL);
  g_autoptr(GtkDirectoryList) directory_list = gtk_directory_list_new("standard::display-name,standard::content-type,standard::icon,standard::size", directory);
  const GError *err = gtk_directory_list_get_error (directory_list);
  if(err) {
    g_print("Error\n");
    g_print("%s\n", err->message);
  } else {
    g_print("No error\n");
  }

  guint list_items = g_list_model_get_n_items(G_LIST_MODEL (directory_list));
  g_print("%s\n", g_get_user_data_dir());
  g_print("%u\n", list_items);
}
