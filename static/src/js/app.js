function App(socket, updateRate, logLevel, mobile){

    this.socket = socket ;
    this.updateRate = updateRate ;
    this.radius = null ;
    this.position = {};
    this.logger = new Logger("App", logLevel);
    if (mobile){
        this.mobile = mobile
    } else {
        this.mobile = false ;
    }

    this.init = function() {
        var self = this;
        this.logger.debug("init: Called");
        this.setupSocket();
        this.setup();
        // this.timer = setInterval(this.update(self), this.updateRate);
    }
    this.updateSocket = function(socket){
        this.socket = socket ;
    }
    this.setupSocket = function(socket){
    }

    this.getPosition = function(callbacks){
        this.logger.debug("getPosition: Called.")
        if (navigator.geolocation){
            navigator.geolocation.getCurrentPosition(
                function(position){
                    var pos = {
                        lat: position.coords.latitude,
                        lon: position.coords.longitude,
                        elevation: 0,
                        continuous: false
                    }
                    callbacks.forEach(function(callback){
                        callback(pos) ;
                    })
                },
                function(error){
                    switch(error.code) {
                        case error.PERMISSION_DENIED:
                            console.log("User denied the request for Geolocation.") ;
                            break;
                        case error.positionITION_UNAVAILABLE:
                            console.log("Location information is unavailable.") ;
                            break;
                        case error.TIMEOUT:
                            console.log("The request to get user location timed out.") ;
                            break;
                        case error.UNKNOWN_ERROR:
                            console.log("An unknown error occurred.")
                            break;
                    }
                    callbacks.forEach(function(callback){
                        callback() ;
                    })
                });
        } else {
            this.logger.error("Browser doesn't support geolocation");
        } ;
    };

    this.makeInitRequest = function(self){
        return function(pos){
            self.logger.debug("makeInitRequest: Called.")
            self.socket.emit("get_planets", {kwargs: {pos: pos, cb_info:{
                cb: "get_planets_cb"
            }}})
        };
    };

    // this.update = function(self){
    //     return function(){
    //         self.planetTracker.update(self.planetTracker)() ;
    //     };
    // };
    this.updateAbout = function(){
        this.logger.debug("updateAbout: Called.")
        var aboutToolTip = d3.select("#about-tooltip");
        aboutToolTip.style("width", d3.select("#planet-plot").style('width'))

    }

    this.setupAbout =  function(){
        var self = this ;
        // ${navigator.userAgent}, ${this.mobile}<br>
        var aboutHTML = `
        <h6>Hover the mouse over objects to see
        their name, current position in the sky, and approximate setting time.<br>
        Setting times are displayed in UTC time.<br>
        Objects that are just outlines are below the horizon.</h6>
        `
        self.aboutTooltipDiv = d3.select("#title-bar").append("div")
            .attr("id","about-tooltip")
            .attr("class", "tooltip")
            .style("opacity", 0.0)
            .style("background","rgb(255,255,255)")
            .style("fill","rgb(255,255,255)")
            .style("width", d3.select("#planet-plot").style('width'))
            .style("max-height", "500px")
            .html(aboutHTML)
            .style('transform', 'translate({}px,{})'.format(0,d3.select("#title").style('height')))

        var aboutDiv = $("#about") ;
        var clicked = false ;
        aboutDiv.on("click", function(){
            clicked = ! clicked ;
            if (clicked){
                self.aboutTooltipDiv.transition()
                    .duration(200)
                    .style("opacity", 1)
                $("#about h4").css("color", "#ce0e25")
            } else {
                self.aboutTooltipDiv.transition()
                    .duration(200)
                    .style("opacity", 0);
                $("#about h4").css("color", "#222")
            }
        });

        // aboutDiv.on(function(){
        //
        // }) ;

    }

    this.setPosition =function(position){
        if (position){
            this.position = position ;
            this.logger.debug("setPosition: new position lat and lon is {}, {}".format(this.position.lat, this.position.lon));
        } else {
            this.logger.error("setPosition: position is undefined.")
        }
    };

    this.setupPlanetTracker = function(position){
        this.planetTracker = new PlanetTracker(this.socket, "#planet-plot",
                                                    $("#planet-plot").width(),
                                                    this.calculatePlotHeight(),
                                                    position, this.logger.level, this.mobile);
        this.planetTracker.setup() ;
        var self = this ;
        $(window).on("resize", function(){
            self.logger.debug("window resized.")
            self.planetTracker.updatePlot({
                width: $("#planet-plot").width(),
                height: self.calculatePlotHeight()
            })
            self.updateAbout();
        });
    }

    this.setup = function(){
        this.setupAbout() ;
        this.getPosition([this.setPosition.bind(this), this.setupPlanetTracker.bind(this)]);
    }

    this.calculatePlotHeight = function(){
        return $(window).height() - $("#title-bar").height();
    }
}

$(document).ready(function(){
    logging.setLevel(logging.levels.DEBUG);
    var mobile = false ;
    if (/Mobi/.test(navigator.userAgent)) {
        mobile = true ;
    }
    var app ;
    var port = location.port;
    var domain = document.domain;
    var socket = io.connect("http://{}:{}".format(domain, port));
    app = new App(socket, 5000, logging.levels.DEBUG, mobile) ;
    socket.on('connect', function(){
        console.info("Updating App's socket connection");
        app.updateSocket(socket);
    })
    app.init()
})
