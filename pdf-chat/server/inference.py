from flask import Flask, request, jsonify, Response
from llama_cpp import Llama

app = Flask(__name__)

llm = Llama(
    model_path="llama-2-13b-chat.ggmlv3.q4_0.bin",
    n_gpu_layers=20000,
    verbose=False,
    n_ctx=2000
)


@app.route("/", methods=["GET", "POST", "PUT", "DELETE"])
def index():
    return jsonify({"message": "pong"})


@app.route('/create_completion', methods=['POST'])
def llama_prompt():
    prompt = request.json['prompt']

    def generate():
        output = llm(prompt=prompt, stream=True, max_tokens=2000)
        for response in output:
            token = response["choices"][0]["text"]
            yield token

    return Response(generate(), mimetype='text/plain')


if __name__ == '__main__':
    app.run(debug=True)
