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

static void gtknotes_window_class_init(GtknotesWindowClass *klass) {
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);

  gtk_widget_class_set_template_from_resource(
      widget_class, "/dev/doodles/gtknotes/gtknotes-window.ui");
}

static void gtknotes_window_init(GtknotesWindow *self) {
  gtk_widget_init_template(GTK_WIDGET(self));
}

void handle_create_note(GtkButton *b) { g_print("Create note\n"); }

void handle_note_text_changed(GtkTextBuffer *buffer) {
  GtkTextIter start;
  GtkTextIter end;

  gtk_text_buffer_get_start_iter(buffer, &start);
  gtk_text_buffer_get_end_iter(buffer, &end);

  g_print("%s\n", gtk_text_buffer_get_text(buffer, &start, &end, FALSE));
}