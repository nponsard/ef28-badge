

#include <WiFi.h>
#include <EFLed.h>
#include <EFLogging.h>
#include "FSMState.h"

#include <algorithm>
#include <WiFi.h>
#include <painlessMesh.h>
#include "mbedtls/base64.h"
#include <DNSServer.h>
// Include certificate data (see note above)
#include "cert.h"
#include "private_key.h"

// We will use wifi
#include <WiFi.h>

namespace Funny
{

    // Replace with your network credentials
    const char *ssid = "Awawi";
    const char *password = "";

    // Set web server port number to 80
    WiFiServer server(80);
    DNSServer dnsServer;

    // Variable to store the HTTP request
    String header;
    Scheduler userScheduler;

    uint8_t own_hue = 0;
    uint8_t eye_color = 0;

    uint8_t rainbow[] = {1, 24, 47, 72, 96, 116, 140, 164, 186, 210, 232};

    void newConnectionCallback(uint32_t nodeId) {}

    void changedConnectionCallback() {}

    void nodeTimeAdjustedCallback(int32_t offset) {}

    void gameLoop() {}

    Task taskGameloop(TASK_SECOND * 1.5, TASK_FOREVER, &gameLoop);

    const char *Funny::getName()
    {
        return "Funny";
    }

    bool Funny::shouldBeRemembered()
    {
        return true;
    }

    void serve()
    {
        WiFiClient client = server.available(); // Listen for incoming clients

        if (client)
        {                                  // If a new client connects,
            Serial.println("New Client."); // print a message out in the serial port
            String currentLine = "";       // make a String to hold incoming data from the client
            while (client.connected())
            { // loop while the client's connected
                if (client.available())
                {                           // if there's bytes to read from the client,
                    char c = client.read(); // read a byte, then
                    Serial.write(c);        // print it out the serial monitor
                    header += c;
                    if (c == '\n')
                    { // if the byte is a newline character
                        // if the current line is blank, you got two newline characters in a row.
                        // that's the end of the client HTTP request, so send a response:
                        if (currentLine.length() == 0)
                        {
                            // HTTP headers always start with a response code (e.g. HTTP/1.1 200 OK)
                            // and a content-type so the client knows what's coming, then a blank line:
                            client.println("HTTP/1.1 200 OK");
                            client.println("Content-type:text/html");
                            client.println("Connection: close");
                            client.println();

                            if (header.indexOf("GET /color/") >= 0)
                            {
                                int index = header.indexOf("GET /color/") + 11;
                                int end_index = header.indexOf(" ", index);
                                String color = header.substring(index, end_index);
                                eye_color = color.toInt();
                            }

                            // turns the GPIOs on and off
                            if (header.indexOf("GET /color") >= 0)
                            {

                                client.println(std::to_string(eye_color).c_str());
                                break;
                            }


                            renderPage();
                            // Break out of the while loop
                            break;
                        }
                        else
                        { // if you got a newline, then clear currentLine
                            currentLine = "";
                        }
                    }
                    else if (c != '\r')
                    {                     // if you got anything else but a carriage return character,
                        currentLine += c; // add it to the end of the currentLine
                    }
                }
            }
            // Clear the header variable
            header = "";
            // Close the connection
            client.stop();
            Serial.println("Client disconnected.");
            Serial.println("");
        }
    }

    void Funny::entry()
    {
        this->tick = 0;

        EFLed.clear();

        WiFi.softAP(ssid, password);

        // Add tasks to scheduler
        userScheduler.addTask(taskGameloop);

        // Start tasks
        taskGameloop.enable();
        server.begin();
        dnsServer.start(53, "*", WiFi.softAPIP());
    }

    void Funny::exit()
    {
        EFLed.clear();
    }

    void Funny::run()
    {
        dnsServer.processNextRequest();

        serve();
        // std::vector<CRGB> dragon = {
        //     CHSV(rainbow[own_hue], 255, 255),
        //     CHSV(rainbow[own_hue], 255, 169),
        //     CHSV(rainbow[own_hue], 255, 124),
        //     CHSV(rainbow[own_hue], 255, 100),
        //     CRGB::Black,
        //     CRGB::Black};

        // EFLed.setAll(dragon.data());

        EFLed.setDragonEye(CHSV(eye_color, 255, 255));

        this->tick++;
    }

    std::unique_ptr<FSMState> Funny::touchEventFingerprintShortpress()
    {
        if (this->isLocked())
        {
            return nullptr;
        }
        return nullptr;
    }

    std::unique_ptr<FSMState> Funny::touchEventFingerprintLongpress()
    {
        return std::make_unique<MenuMain>();
    }

    std::unique_ptr<FSMState> Funny::touchEventFingerprintRelease()
    {
        if (this->isLocked())
        {
            return nullptr;
        }

        this->globals->huemeshOwnHue = own_hue;
        this->is_globals_dirty = true;
        this->tick = 0;

        return nullptr;
    }

    std::unique_ptr<FSMState> Funny::touchEventNoseShortpress()
    {
        return nullptr;
    }

    std::unique_ptr<FSMState> Funny::touchEventNoseLongpress()
    {
        return nullptr;
    }

    std::unique_ptr<FSMState> Funny::touchEventNoseRelease()
    {

        return nullptr;
    }

    std::unique_ptr<FSMState> Funny::touchEventAllLongpress()
    {
        this->toggleLock();
        return nullptr;
    }

    char *renderPage()
    {
        const char *start = R"(
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello</title>

</head>

<body>

    <!-- add an image with base64 data -->

    <img
        src="data:image/webp;base64,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" />

    <h1>Hey ! You found sautax's badge !</h1>
    <p>
        Did you know that all supersponsor badges have wifi capability ?
    </p>

    <p>
        Want to find me ? Change the color of the eye on my badge ! (only hue will change)
    </p>
    <input type="color" id="color" name="color" value="#ff0000">

    <p>
        Current color :
    <div id="current" style="width: 50px; height: 50px; background-color: hsl(11, 50%, 50%);"></div>
    </p>

    <script>

        function hexToHSL(H) {
            // Convert hex to RGB first
            let r = 0, g = 0, b = 0;
            if (H.length == 4) {
                r = "0x" + H[1] + H[1];
                g = "0x" + H[2] + H[2];
                b = "0x" + H[3] + H[3];
            } else if (H.length == 7) {
                r = "0x" + H[1] + H[2];
                g = "0x" + H[3] + H[4];
                b = "0x" + H[5] + H[6];
            }
            // Then to HSL
            r /= 255;
            g /= 255;
            b /= 255;
            let cmin = Math.min(r, g, b),
                cmax = Math.max(r, g, b),
                delta = cmax - cmin,
                h = 0,
                s = 0,
                l = 0;

            if (delta == 0)
                h = 0;
            else if (cmax == r)
                h = ((g - b) / delta) % 6;
            else if (cmax == g)
                h = (b - r) / delta + 2;
            else
                h = (r - g) / delta + 4;

            h = Math.round(h * 60);

            if (h < 0)
                h += 360;

            return h;
        }
        const current = document.getElementById("current");


        function setColor(e) {
            const color = e.target.value;
            console.log(color);

            const hsl = hexToHSL(color);
            console.log(hsl);
            // GET ?
            // Yes I know I'm lazy
            fetch(`${window.location.origin}/color/${hsl}`)
                .then(response => {
                    console.log(response);
                    current.style.backgroundColor = `hsl(${hsl}, 50%, 50%)`;

                })
                .catch(error => {
                    console.error(error);
                    window.location.reload();
                });
        }

        document.getElementById("color").addEventListener("change", setColor);

        fetch(`${window.location.origin}/color`)
            .then(response => response.text())
            .then(hsl => {
                current.style.backgroundColor = `hsl(${hsl}, 50%, 50%)`;
            })
            .catch(error => {
                console.error(error);
            });





    </script>
</body>

</html>

        )";
    }

}
