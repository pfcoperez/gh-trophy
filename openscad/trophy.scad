include <activity-data.scad>

$fn = 100;

baseH = 10;
baseL = 155;
baseW = 40;

activityBase = 2; 
activityL = 3;
activityHfactor = activityL*5;
activityPadding = 10;

ghLogoSizeFactor = 0.05;
ghLogoH = 1;
ghLogoX = 2.5;

fontSize = 6;

module ghLogo() {
    translate([ghLogoX, -6, baseH+ghLogoH]) {
        scale([ghLogoSizeFactor, ghLogoSizeFactor, 1]) {
            linear_extrude(ghLogoH) import("images/github.svg");
        }
    }
} 

module userHandle() {
    translate([20.5+ghLogoX, -1, baseH+ghLogoH]) {
        linear_extrude(ghLogoH) text(ghHandleTxt, fontSize);
    }
}

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
            h = activityBase + log(week[i])*activityHfactor;
            translate([j*activityL, (len(week)-i)*activityL, baseH]) {
                cube([activityL, activityL, h]);
            }
        }
    }
}

module date() {
    translate([90, -1, baseH+ghLogoH]) {
        linear_extrude(ghLogoH) text(spanTxt, fontSize);
    }
}

union() {
    ghLogo();
    base();
    translate([0, activityPadding, 0]) {
        activity();
    }
    userHandle();
    date();
}