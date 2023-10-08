// You installed the `express` library earlier. For more information, see "[JavaScript example: Install dependencies](#javascript-example-install-dependencies)."
const express = require("express");

// This initializes a new Express application.
var router = express.Router();

// This defines a POST route at the `/webhook` path. This path matches the path that you specified for the smee.io forwarding. For more information, see "[Forward webhooks](#forward-webhooks)."
//
// Once you deploy your code to a server and update your webhook URL, you should change this to match the path portion of the URL for your webhook.
router.post(
  "/webhook",
  express.json({ type: "application/json" }),
  (request, response) => {
    // Respond to indicate that the delivery was successfully received.
    // Your server should respond with a 2XX response within 10 seconds of receiving a webhook delivery. If your server takes longer than that to respond, then GitHub terminates the connection and considers the delivery a failure.
    response.status(202).send("Accepted");

    // Check the `x-github-event` header to learn what event type was sent.
    const githubEvent = request.headers["x-github-event"];

    // You should add logic to handle each event type that your webhook is subscribed to.
    // For example, this code handles the `issues` and `ping` events.
    //
    // If any events have an `action` field, you should also add logic to handle each action that you are interested in.
    // For example, this code handles the `opened` and `closed` actions for the `issue` event.
    //
    // For more information about the data that you can expect for each event type, see "[AUTOTITLE](/webhooks/webhook-events-and-payloads)."
    if (githubEvent === "push") {
      const data = request.body;
      const action = data.action;
      console.log(">> action", action);
    } else if (githubEvent === "ping") {
      console.log("GitHub sent the ping event");
    } else {
      console.log(`Unhandled event: ${githubEvent}`);
    }
  }
);

module.exports = router;
