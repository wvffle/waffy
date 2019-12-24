#include <stdio.h>

#include <gtk/gtk.h>
#include <gdk/gdkwayland.h>
#include <gtk-layer-shell/gtk-layer-shell.h>
#include <gio/gdesktopappinfo.h>

#include "find_desktop.h"
#include "filter.h"

#define COLUMNS 4
#define ICON_SIZE 48

desktop_entry_batch* all_desktop_entries = NULL;
desktop_entry_batch* filtered = NULL;
GtkGrid* app_grid = NULL;
GtkGrid* fav_grid = NULL;
GtkWidget *search_input;
uint current_items = 0;
int current_item = 0;
int current_item_kb = 0;

// HACK: Get over with the fact that after changing CSS class, button gets rehovered
int last_current_item = 0;
int last_current_item_kb = 0;

GdkCursor* arrow;
GdkCursor* pointer;

void window_destroy (GtkWidget* widget, gpointer *data) {
    deb_destructor(all_desktop_entries);
    gtk_main_quit();
}

void update_current () {
    int idx = -1;

    if (current_item != last_current_item) {
        last_current_item = current_item;
        last_current_item_kb = current_item;
        current_item_kb = current_item;
        idx = current_item;
    } else if (current_item_kb != last_current_item_kb) {
        last_current_item_kb = current_item_kb;
        idx = current_item_kb;
    }

    if (idx == -1) return;
    for (size_t i = 0; i < current_items; ++i) {
        GtkWidget* app = gtk_grid_get_child_at(app_grid, i % COLUMNS, i / COLUMNS);
        if (app == NULL) continue;
        GtkStyleContext* ctx = gtk_widget_get_style_context(app);

        if (gtk_style_context_has_class(ctx, "active") == TRUE) {
            gtk_style_context_remove_class(ctx, "active");
        }

        if (i == idx) {
            gtk_style_context_add_class(ctx, "active");
        }
    }
}

void app_enter (GtkWidget* widget, GdkEvent* event, int* data) {
    gdk_window_set_cursor(gtk_widget_get_window(widget), pointer);
    current_item = *data;
    update_current();
}

void app_leave (GtkWidget* widget, GdkEvent* event, int* data) {
    gdk_window_set_cursor(gtk_widget_get_window(widget), arrow);
    current_item = *data;
    update_current();
}

void app_clicked (GtkButton* button, desktop_entry* entry) {
    GDesktopAppInfo* info = g_desktop_app_info_new(entry->gtk_launch_name);

    if (info != NULL) {
        g_autoptr(GError) error = NULL;
        g_app_info_launch(G_APP_INFO(info), NULL, NULL, &error);

        if (error != NULL) {
            g_error("Could not run program '%s': %s", entry->name, error->message);
            window_destroy(NULL, NULL);
            exit(EXIT_FAILURE);
        }

        window_destroy(NULL, NULL);
        exit(EXIT_SUCCESS);
    }

    window_destroy(NULL, NULL);
    exit(EXIT_FAILURE);
}

void update_apps (desktop_entry_batch* apps) {
    // Reset grid
    for (size_t i = 0; i < current_items; ++i) {
        GtkWidget* app = gtk_grid_get_child_at(app_grid, i % COLUMNS, i / COLUMNS);
        gtk_container_remove(GTK_CONTAINER(app_grid), app);
    }

    int old_current_items = current_items;
    if (apps == NULL || (current_items = apps->length) == 0) {
        current_items = 0;
        return;
    }

    if (old_current_items != current_items) {
        current_item = 0;
        current_item_kb = 0;
        update_current();
    }

    int n = 0;
    desktop_entry_batch_node* curr = apps->first;
    while (curr != NULL) {
        GtkIconTheme* theme = gtk_icon_theme_get_default();
        GtkWidget* icon = NULL;
        GdkPixbuf* pixbuf = NULL;

        if (is_path(curr->entry->icon)) {
            GtkWidget* image = gtk_image_new_from_file(curr->entry->icon);
            pixbuf = gtk_image_get_pixbuf(GTK_IMAGE(image));
//            gtk_widget_destroy(image);
        } else {
            const gchar** icons = g_themed_icon_get_names((GThemedIcon *) g_themed_icon_new(curr->entry->icon));
            GtkIconInfo* info = gtk_icon_theme_choose_icon(theme, icons, ICON_SIZE, GTK_ICON_LOOKUP_FORCE_SIZE);

            if (info != NULL) {
                GtkWidget* image = gtk_image_new_from_file(gtk_icon_info_get_filename(info));
                pixbuf = gtk_image_get_pixbuf(GTK_IMAGE(image));
            }

        }

        if (GDK_IS_PIXBUF(pixbuf)) {
            pixbuf = gdk_pixbuf_scale_simple(pixbuf, ICON_SIZE, ICON_SIZE, GDK_INTERP_TILES);
            icon = gtk_image_new_from_pixbuf(pixbuf);
        }

        GtkWidget* app = gtk_button_new_with_label(curr->entry->name);

        if (icon != NULL) {
            gtk_button_set_image(GTK_BUTTON(app), icon);
            gtk_widget_set_halign(icon, GTK_ALIGN_START);
        }

        gtk_widget_set_hexpand(app, FALSE);

        uint col = n % COLUMNS;
        uint row = n / COLUMNS;
        gtk_grid_attach(app_grid, app, col, row, 1, 1);
        gtk_widget_show_all(app);

        int* pos = malloc(sizeof(int));
        memcpy(pos, &n, sizeof(int));
        g_signal_connect(app, "clicked", G_CALLBACK(app_clicked), curr->entry);
        g_signal_connect(app, "enter-notify-event", G_CALLBACK(app_enter), pos);
        g_signal_connect(app, "leave-notify-event", G_CALLBACK(app_leave), pos);

        curr = curr->next;
        n += 1;
    }

    update_current();
}

gboolean key_pressed (GtkWidget* window, GdkEventKey* event, gpointer data) {
    if (event->keyval == GDK_KEY_Escape) {
        window_destroy(window, data);
        return TRUE;
    }

    if (event->keyval == GDK_KEY_Return) {
        desktop_entry_batch* apps = filtered;
        if (apps == NULL) apps = all_desktop_entries;

        desktop_entry_batch_node* curr = apps->first;
        int n = 0;
        while (curr != NULL) {
            if (n == current_item) {
                app_clicked(NULL, curr->entry);
                return TRUE;
            }

            curr = curr->next;
            n += 1;
        }

        return TRUE;
    }

    if (event->keyval == GDK_KEY_Left) {
        if (current_item_kb == 0) return TRUE;
        current_item_kb -= 1;
//        update_current();
        return TRUE;
    }

    if (event->keyval == GDK_KEY_Right) {
        if (current_item_kb == current_items - 1) return TRUE;
        current_item_kb += 1;
//        update_current();
        return TRUE;
    }

    if (event->keyval == GDK_KEY_Up) {
        current_item_kb -= COLUMNS;
        if (current_item_kb < 0) current_item_kb += COLUMNS;

//        update_current();
        return TRUE;
    }

    if (event->keyval == GDK_KEY_Down) {
        current_item_kb += COLUMNS;
        if (current_item_kb > current_items - 1) current_item_kb -= COLUMNS;

//        update_current();
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
    filtered = filter_apps(all_desktop_entries, needle, FUZZY);
    update_apps(filtered);

    if (filtered != all_desktop_entries && filtered != NULL) {
        deb_destructor_no_entry(filtered);
    }
}

int main (int argc, char* argv[]) {
    all_desktop_entries = find_all_desktop_files();

    gtk_init(&argc, &argv);

    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_layer_init_for_window(GTK_WINDOW(window));
    gtk_layer_set_layer (GTK_WINDOW(window), GTK_LAYER_SHELL_LAYER_OVERLAY);
    gtk_layer_set_keyboard_interactivity (GTK_WINDOW(window), TRUE);

    // HACK: Set window fullscreen
    gtk_layer_set_anchor (GTK_WINDOW(window), 0, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 1, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 2, TRUE);
    gtk_layer_set_anchor (GTK_WINDOW(window), 3, TRUE);

    gtk_widget_realize(window);

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
    gtk_widget_set_name(search_input, "search");
    gtk_box_pack_start(search_box, search_input, TRUE, TRUE, 0);

    // Apps grid layout
    app_grid = (GtkGrid *) gtk_grid_new();
    gtk_box_pack_start(layout, GTK_WIDGET(app_grid), TRUE, TRUE, 0);
    for (size_t i = 0; i < COLUMNS; ++i) {
        gtk_grid_insert_column(app_grid, i);
    }
    gtk_grid_set_column_spacing(app_grid, 17);
    gtk_grid_set_row_spacing(app_grid, 17);
    gtk_widget_set_name(GTK_WIDGET(app_grid), "apps");

    gtk_widget_set_halign(app_grid, GTK_ALIGN_CENTER);

    update_apps(all_desktop_entries);

    // Set window style
    // TODO: Respect pywal
    GtkCssProvider* css_provider = gtk_css_provider_new();

    if (gtk_css_provider_load_from_path(css_provider, "../assets/main.css", NULL)) {
        gtk_style_context_add_provider_for_screen(gtk_widget_get_screen(window),
                                                  GTK_STYLE_PROVIDER(css_provider),
                                                  GTK_STYLE_PROVIDER_PRIORITY_USER);
    }

    GdkDisplay* display = gtk_widget_get_display(window);
    arrow = GDK_CURSOR(gdk_cursor_new_from_name(display, "default"));
    pointer = GDK_CURSOR(gdk_cursor_new_from_name(display, "pointer"));

    gtk_window_set_title(GTK_WINDOW(window), "Waffy");
    gtk_widget_show_all(window);
    gtk_main();
}
