#include <stdio.h>
#include <stdlib.h>

#include <gtk-3.0/gtk/gtk.h>

void window_destroy (GtkWidget* window, gpointer data) {
    gtk_main_quit();
}

int main (int argc, char* argv[]) {
    GtkWidget* window;
    GtkWidget* button;
    gtk_init(&argc, &argv);

    window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_keep_above(GTK_WINDOW(window), 1);

    g_signal_connect(window, "destroy", G_CALLBACK(window_destroy), NULL);
    gtk_container_set_border_width(GTK_CONTAINER(window), 20);

    gtk_widget_show_all(window);
    gtk_main();
}
