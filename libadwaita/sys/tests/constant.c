// Generated by gir (https://github.com/gtk-rs/gir @ 3c6ebf4)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21f7670)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_LOOSE);
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_STRICT);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_ALWAYS);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_AUTO);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_NEVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_BACK);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_FORWARD);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_CROSSFADE);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_NONE);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_AUTO);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_NARROW);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_WIDE);
    return 0;
}
