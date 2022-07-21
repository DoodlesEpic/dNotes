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
GtkFileChooserAction action = GTK_FILE_CHOOSER_ACTION_SAVE;

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
      "Save File", GTK_WINDOW(&parent_instance), action, "_Save", "_Cancel");
  chooser = GTK_FILE_CHOOSER(native);
}

void handle_note_text_changed(GtkTextBuffer *buffer) { note_buffer = buffer; }

void handle_create_note(GtkButton *b) {
  GtkTextIter start;
  GtkTextIter end;

  gtk_text_buffer_get_start_iter(note_buffer, &start);
  gtk_text_buffer_get_end_iter(note_buffer, &end);

  g_print("%s\n", gtk_text_buffer_get_text(note_buffer, &start, &end, FALSE));

  // Show the native file dialog we created on window init
  gtk_file_chooser_set_current_name(chooser, "note.txt");
  g_signal_connect(native, "response", G_CALLBACK(on_response), NULL);
  gtk_native_dialog_show(GTK_NATIVE_DIALOG(native));
}

static void on_response(GtkNativeDialog *native, int response) {
  if (response == GTK_RESPONSE_ACCEPT) {
    chooser = GTK_FILE_CHOOSER(native);
    GFile *file = gtk_file_chooser_get_file(chooser);

    // TODO: Implement save to file
    // save_to_file(file);

    g_object_unref(file);
  }
}