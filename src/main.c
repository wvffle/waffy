#include <stdio.h>
#include <stdlib.h>

#include <gtk/gtk.h>
#include <gdk/gdkwayland.h>
#include <gtk-layer-shell/gtk-layer-shell.h>

#include "find_desktop.h"

void window_destroy (GtkWidget* window, gpointer data) {
    gtk_main_quit();
}

gboolean key_pressed (GtkWidget* window, GdkEventKey* event, gpointer data) {
    if (event->keyval == GDK_KEY_Escape) {
        gtk_main_quit();
    }
}

int main (int argc, char* argv[]) {
/*
    desktop_entry_batch* deb = find_all_desktop_files();

    desktop_entry_batch_node* curr = deb->first;
    while (curr != NULL) {
        curr = curr->next;
    }

    deb_destructor(deb);
*/
    gtk_init(&argc, &argv);

    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_layer_init_for_window(GTK_WINDOW(window));
    gtk_layer_set_layer (GTK_WINDOW(window), GTK_LAYER_SHELL_LAYER_TOP);
    gtk_layer_set_keyboard_interactivity (GTK_WINDOW(window), TRUE);

    // HACK: Set fullscreen
    gtk_layer_set_anchor (GTK_WINDOW(window), 0, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 1, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 2, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 3, TRUE);

    gtk_widget_realize(window);
    gtk_widget_set_name(window, "window");

    // Set window options
    gtk_window_set_resizable(GTK_WINDOW(window), FALSE);
    gtk_window_set_decorated(GTK_WINDOW(window), FALSE);

    g_signal_connect(window, "destroy", G_CALLBACK(window_destroy), NULL);
    g_signal_connect(window, "key_press_event", G_CALLBACK(key_pressed), NULL);

    gtk_window_set_title(GTK_WINDOW(window), "Waffy");
    gtk_widget_show_all(window);
    gtk_main();
}
