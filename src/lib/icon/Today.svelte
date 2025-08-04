<script lang="ts">
  interface Props {
    day: number;
    width?: string;
    height?: string;
  }

  const { day, width, height }: Props = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 23 20"
  {width}
  {height}
  class="iconify"
>
  <g>
    <path
      fill-rule="evenodd"
      d="M2.893.688a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h13.922a2 2 0 0 0 1.476-.651l4.078-4.46a2 2 0 0 0 .524-1.35v-9.54a2 2 0 0 0-2-2z"
      clip-rule="evenodd"
    ></path>
    <text
      x="50%"
      y="50%"
      fill="#fff"
      font-size="10"
      font-weight="bold"
      style="dominant-baseline:middle"
      text-anchor="middle"
    >
      {day}
    </text>
  </g>
</svg>
