#include <stdio.h>

#include <gtk/gtk.h>
#include <gdk/gdkwayland.h>
#include <gtk-layer-shell/gtk-layer-shell.h>

#include "find_desktop.h"
#include "filter.h"

#define COLUMNS 4

desktop_entry_batch* all_desktop_entries = NULL;
GtkGrid* app_grid = NULL;
GtkGrid* fav_grid = NULL;
GtkWidget *search_input;
uint current_items = 0;

void window_destroy (GtkWidget* window, gpointer data) {
    deb_destructor(all_desktop_entries);
    gtk_main_quit();
}

void update_apps (desktop_entry_batch* apps) {
    // Reset grid
    for (size_t i = 0; i < current_items; ++i) {
        GtkWidget* app = gtk_grid_get_child_at(app_grid, i % COLUMNS, i / COLUMNS);
        gtk_container_remove(GTK_CONTAINER(app_grid), app);
    }

    if (apps == NULL || (current_items = apps->length) == 0) {
        current_items = 0;
        return;
    }

    int n = 0;
    desktop_entry_batch_node* curr = apps->first;
    while (curr != NULL) {
        GtkWidget* app = gtk_label_new(curr->entry->name);
        uint col = n % COLUMNS;
        uint row = n / COLUMNS;
        gtk_grid_attach(app_grid, app, col, row, 1, 1);
        gtk_widget_show(app);

        curr = curr->next;
        n += 1;
    }
}

gboolean key_pressed (GtkWidget* window, GdkEventKey* event, gpointer data) {
    if (event->keyval == GDK_KEY_Escape) {
        window_destroy(window, data);
        return TRUE;
    }

    if (event->keyval == GDK_KEY_Return) {
        // TODO: Launch first app
        window_destroy(window, data);
        return TRUE;
    }

    return FALSE;
}
gboolean key_released (GtkWidget* window, GdkEventKey* event, gpointer data) {
    // Normal key input
    GtkTextBuffer* buf = gtk_text_view_get_buffer(GTK_TEXT_VIEW(search_input));
    GtkTextIter start;
    GtkTextIter  end;
    gtk_text_buffer_get_bounds (buf, &start, &end);

    char* needle = gtk_text_buffer_get_text(buf, &start, &end, FALSE);
    desktop_entry_batch* filtered = filter_apps(all_desktop_entries, needle, FUZZY);
    update_apps(filtered);

    if (filtered != all_desktop_entries && filtered != NULL) {
        deb_destructor_no_entry(filtered);
    }
}

int main (int argc, char* argv[]) {
    all_desktop_entries = find_all_desktop_files();
    desktop_entry_batch_node* curr = all_desktop_entries->first;

    while (curr != NULL) {
        curr = curr->next;
    }

    gtk_init(&argc, &argv);

    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_layer_init_for_window(GTK_WINDOW(window));
    gtk_layer_set_layer (GTK_WINDOW(window), GTK_LAYER_SHELL_LAYER_TOP);
    gtk_layer_set_keyboard_interactivity (GTK_WINDOW(window), TRUE);

    // HACK: Set window fullscreen
    gtk_layer_set_anchor (GTK_WINDOW(window), 0, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 1, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 2, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 3, TRUE);

    gtk_widget_realize(window);
    gtk_widget_set_name(window, "window");

    // Set window options
    gtk_window_set_resizable(GTK_WINDOW(window), FALSE);
    gtk_window_set_decorated(GTK_WINDOW(window), FALSE);

    // Add event handlers
    g_signal_connect(window, "destroy", G_CALLBACK(window_destroy), NULL);
    g_signal_connect(window, "key_press_event", G_CALLBACK(key_pressed), NULL);
    g_signal_connect(window, "key_release_event", G_CALLBACK(key_released), NULL);

    // Main layout
    GtkBox* layout = GTK_BOX(gtk_box_new(GTK_ORIENTATION_VERTICAL, 0));
    gtk_container_add(GTK_CONTAINER(window), GTK_WIDGET(layout));

    // Add search field
    GtkBox* search_box = GTK_BOX(gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0));
    gtk_box_pack_start(layout, GTK_WIDGET(search_box), FALSE, FALSE, 0);

    GtkWidget* search_label = gtk_label_new(NULL);
    gtk_label_set_markup(GTK_LABEL(search_label), "<span>Search:    </span>");
    gtk_box_pack_start(search_box, search_label, FALSE, FALSE, 0);

    search_input = gtk_text_view_new();
    gtk_box_pack_start(search_box, search_input, TRUE, TRUE, 0);

    // Apps grid layout
    app_grid = (GtkGrid *) gtk_grid_new();
    gtk_box_pack_start(layout, GTK_WIDGET(app_grid), TRUE, TRUE, 0);
    for (size_t i = 0; i < COLUMNS; ++i) {
        gtk_grid_insert_column(app_grid, i);
    }

    update_apps(all_desktop_entries);

    // Set window style
    GtkCssProvider* css_provider = gtk_css_provider_new();
    if (gtk_css_provider_load_from_path(css_provider, "assets/main.css", NULL)) {
        gtk_style_context_add_provider(gtk_widget_get_style_context(window),
                                       GTK_STYLE_PROVIDER(css_provider),
                                       GTK_STYLE_PROVIDER_PRIORITY_USER);

        gtk_style_context_add_provider(gtk_widget_get_style_context(app_grid),
                                       GTK_STYLE_PROVIDER(css_provider),
                                       GTK_STYLE_PROVIDER_PRIORITY_USER);
    }

    gtk_window_set_title(GTK_WINDOW(window), "Waffy");
    gtk_widget_show_all(window);
    gtk_main();
}
