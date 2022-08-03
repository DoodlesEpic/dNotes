/* gtknotes-window.h
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

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTKNOTES_TYPE_WINDOW (gtknotes_window_get_type())

G_DECLARE_FINAL_TYPE(GtknotesWindow, gtknotes_window, GTKNOTES, WINDOW,
                     GtkApplicationWindow)

void handle_create_note(G_GNUC_UNUSED GtkButton *b);
void handle_note_text_changed(GtkTextBuffer *buffer);

void on_response(GtkNativeDialog *native, int response);
void save_to_file(GFile *file);

void list_notes();

G_END_DECLS
