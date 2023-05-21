import random
import time
import requests
import xmltodict
import json
from paho.mqtt import client as mqtt_client

time.sleep(10)

client = mqtt_client.Client(f"pool-mqtt-{random.randint(0, 1000)}")
client.connect("192.168.12.100", 1883)
client.loop_start()

while True:
    ph = xmltodict.parse(requests.get("http://192.168.12.105/cgi-bin/webgui.fcgi?xmlitem=34.4001").text)["pm5"]["item"]["@value"]

    cl = xmltodict.parse(requests.get("http://192.168.12.105/cgi-bin/webgui.fcgi?xmlitem=34.4008").text)["pm5"]["item"]["@value"]

    status = {
        "pH": float(ph),
        "Cl": float(cl)
    }

    status_str = json.dumps(status)

    client.publish("shellies/status/pool", status_str)
    print("Published")
    time.sleep(60)
