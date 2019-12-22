//
// Created by waff on 12/20/19.
//

#ifndef WAFFY_DESKTOP_ENTRY_H
#define WAFFY_DESKTOP_ENTRY_H

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

typedef struct desktop_entry {
    char gtk_launch_name[FILENAME_MAX];
    char name[32];
    char icon[PATH_MAX];
} desktop_entry;

typedef struct desktop_entry_batch_node {
    struct desktop_entry* entry;
    struct desktop_entry_batch_node* next;
    struct desktop_entry_batch_node* prev;
} desktop_entry_batch_node;

typedef struct desktop_entry_batch {
    size_t length;
    struct desktop_entry_batch_node* first;
    struct desktop_entry_batch_node* last;
} desktop_entry_batch;

desktop_entry* de_constructor (const char* gtk_launch_name);

desktop_entry_batch* deb_constructor ();
void deb_destructor (desktop_entry_batch* batch);
void deb_destructor_no_entry(desktop_entry_batch *batch);


size_t deb_push (desktop_entry_batch* batch, desktop_entry* entry);
size_t deb_unshift (desktop_entry_batch* batch, desktop_entry* entry);

desktop_entry* deb_pop (desktop_entry_batch* batch);
desktop_entry* deb_shift (desktop_entry_batch* batch);

void deb_concat (desktop_entry_batch* target, desktop_entry_batch* source);

#endif //WAFFY_DESKTOP_ENTRY_H
