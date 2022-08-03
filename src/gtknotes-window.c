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

static GtkTextBuffer *note_buffer;
static GtkFileChooserNative *native;
static GtkFileChooser *chooser;

static void gtknotes_window_class_init(GtknotesWindowClass *klass) {
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
  gtk_widget_class_set_template_from_resource(widget_class, "/dev/doodles/gtknotes/gtknotes-window.ui");
}

static void gtknotes_window_init(GtknotesWindow *self) {
  gtk_widget_init_template(GTK_WIDGET(self));
  GtkApplicationWindow parent_instance = self->parent_instance;

  // Create the native dialog to be shown when saving the note
  native = gtk_file_chooser_native_new("Save File", GTK_WINDOW(&parent_instance), GTK_FILE_CHOOSER_ACTION_SAVE, "_Save", "_Cancel");
  chooser = GTK_FILE_CHOOSER(native);
  g_signal_connect(native, "response", G_CALLBACK(on_response), NULL);
}

void handle_note_text_changed(GtkTextBuffer *buffer) {
  note_buffer = buffer;
  list_notes();
}

void handle_create_note(G_GNUC_UNUSED GtkButton *b) {
  // Show the native file dialog we created on window init
  gtk_file_chooser_set_current_name(chooser, "note.txt");
  gtk_native_dialog_show(GTK_NATIVE_DIALOG(native));
}

void on_response(GtkNativeDialog *native, int response) {
  g_assert(GTK_IS_NATIVE_DIALOG(native));

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

// Enumerates the notes present in the data directory for the application
// Currently this is called in every change to the text field for testing.
void list_notes() {
  g_autoptr(GDir) directory = g_dir_open(g_get_user_data_dir(), NULL, NULL);
  g_autofree const gchar *filename;
  while ((filename = g_dir_read_name(directory))) {
    printf("%s\n", filename);
  }
}
