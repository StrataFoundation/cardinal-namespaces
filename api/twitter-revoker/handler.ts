/* eslint-disable @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-argument */
import * as twitterApprover from "./twitter-revoker";

module.exports.revoke = async (event) => {
  try {
    const txid = await twitterApprover.revokeHolder(
      event?.queryStringParameters?.tweetId,
      event?.queryStringParameters?.publicKey,
      event?.queryStringParameters?.handle,
      event?.queryStringParameters?.cluster
    );
    return {
      statusCode: 200,
      body: JSON.stringify({ result: "done", txid }),
    };
  } catch (e) {
    console.log("Error: ", e);
    return {
      statusCode: 400,
      // eslint-disable-next-line @typescript-eslint/no-unsafe-call
      body: JSON.stringify({ error: e.toString() }),
    };
  }
};
