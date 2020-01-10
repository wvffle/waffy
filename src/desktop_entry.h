//
// Created by waff on 12/20/19.
//

#ifndef WAFFY_DESKTOP_ENTRY_H
#define WAFFY_DESKTOP_ENTRY_H

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

// Desktop entry structure
typedef struct desktop_entry {
    char gtk_launch_name[FILENAME_MAX]; // Used to find GtkAppInfo
    char name[32];                      // App display name
    char icon[PATH_MAX];                // App icon
} desktop_entry;

// Two-way list node
typedef struct desktop_entry_batch_node {
    struct desktop_entry* entry;            // Desktop entry
    struct desktop_entry_batch_node* next;  // Next node
    struct desktop_entry_batch_node* prev;  // Previous node
} desktop_entry_batch_node;

// Two-way list starter
typedef struct desktop_entry_batch {
    size_t length;                          // List length
    struct desktop_entry_batch_node* first; // First node on the list
    struct desktop_entry_batch_node* last;  // Last node on the list
} desktop_entry_batch;

// Helper function to create new desktop entry and allocate memory
desktop_entry* de_constructor (const char* gtk_launch_name);

// Helper function to create new list and allocate memory
desktop_entry_batch* deb_constructor ();

// Helper function to clean the list
void deb_destructor (desktop_entry_batch* batch);

// Helper function to clean the list without cleaning entries
void deb_destructor_no_entry(desktop_entry_batch* batch);


// Helper function to add entry to the end of a list
size_t deb_push (desktop_entry_batch* batch, desktop_entry* entry);

// Helper function to add entry to the beginning of a list
size_t deb_unshift (desktop_entry_batch* batch, desktop_entry* entry);

// Helper function to remove entry from the end of a list
desktop_entry* deb_pop (desktop_entry_batch* batch);

// Helper function to remove entry from the beginning of a list
desktop_entry* deb_shift (desktop_entry_batch* batch);

// Helper function to join source to target list. Invalidates source list
void deb_concat (desktop_entry_batch* target, desktop_entry_batch* source);

#endif //WAFFY_DESKTOP_ENTRY_H
