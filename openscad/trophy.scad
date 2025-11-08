include <activity-data.scad>

$fn = 100;

baseH = 10;
baseL = 155;
baseW = 40;

activityBase = 2; 
activityL = 3;
activityHfactor = activityL;
activityPadding = 10;

module base() {
    minkowski() {
        cube([baseL, baseW, baseH]);
        cylinder(h = 1, r = baseH);
    }
}

module activity() {
    for (j = [0:len(rawActivity)-1]) {
        week = rawActivity[j];
        for (i = [0:len(week)-1]) {
            h = activityBase + week[i]*activityHfactor;
            translate([j*activityL, (len(week)-i)*activityL, baseH]) {
                cube([activityL, activityL, h]);
            }
        }
    }
}

module userHandle() {
}

module year() {
}

union() {
    base();
    translate([0, activityPadding, 0]) {
        activity();
    }
    userHandle();
    year();
}